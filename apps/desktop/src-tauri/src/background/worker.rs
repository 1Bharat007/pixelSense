use crate::background::config::BackgroundConfig;
use crate::background::error::BackgroundError;
use crate::background::event::models::{AdaptiveEvent, AdaptiveEventKind, EventPriority};
use crate::background::event::queue::EventQueue;
use crate::background::models::{
    now_ms, BackgroundDiagnostics, PipelineProfile, PipelineResult, WorkerHealth, WorkerId,
    WorkerState,
};
use crate::background::profiler::PipelineProfiler;
use crate::background::scheduler::PollingScheduler;
use crate::background::service::Service;
use crate::performance::manager::PerformanceManager;
use crate::performance::models::PerformanceState;
use crate::experience::history::manager::HistoryManager;
use crate::experience::multi_monitor::scheduler::MultiMonitorScheduler;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

/// The adaptive comfort pipeline loop.
///
/// ## Responsibilities
/// - Run continuously on a dedicated `std::thread`.
/// - On each cycle: assemble context → calculate → recommend → execute.
/// - Update `WorkerHealth` heartbeat every loop.
/// - Update `WorkerHealth` cycle timestamp only after a complete pipeline.
/// - Isolate every subsystem failure — never crash on partial failure.
///
/// ## Non-Responsibilities
/// - Does NOT manage its own lifecycle (that is ServiceManager's domain).
/// - Does NOT restart itself (that is WorkerWatchdog's domain).
/// - Does NOT communicate with the UI directly.
///
/// ## Safe Shutdown
/// When `cancel_token` is set to true, the worker finishes the current pipeline
/// cycle completely before stopping. It will never terminate during a hardware
/// brightness write.
pub struct BackgroundWorker {
    id: WorkerId,
    config: BackgroundConfig,
    cancel_token: Arc<AtomicBool>,
    health: Arc<Mutex<WorkerHealth>>,
    event_queue: Arc<EventQueue>,
    profiler: Arc<PipelineProfiler>,
    performance_manager: Arc<PerformanceManager>,
    history_manager: Arc<HistoryManager>,
    multi_monitor_scheduler: Arc<MultiMonitorScheduler>,
}

impl BackgroundWorker {
    pub fn new(
        config: BackgroundConfig, 
        performance_manager: Arc<PerformanceManager>,
        history_manager: Arc<HistoryManager>,
        multi_monitor_scheduler: Arc<MultiMonitorScheduler>
    ) -> Self {
        let id = WorkerId::new("background_adaptive_worker");
        let health = WorkerHealth::initial(id.clone(), config.base_poll_interval_ms);

        Self {
            id,
            config,
            cancel_token: Arc::new(AtomicBool::new(false)),
            health: Arc::new(Mutex::new(health)),
            event_queue: Arc::new(EventQueue::new()),
            profiler: Arc::new(PipelineProfiler::new()),
            performance_manager,
            history_manager,
            multi_monitor_scheduler,
        }
    }

    /// Signal an external event into the worker's queue.
    pub fn enqueue_event(&self, kind: AdaptiveEventKind, priority: EventPriority) {
        self.event_queue.push(AdaptiveEvent::new(kind, priority));
    }

    /// Read the latest health snapshot. Non-blocking.
    pub fn get_health(&self) -> WorkerHealth {
        self.health
            .lock()
            .map(|h| h.clone())
            .unwrap_or_else(|_| WorkerHealth::initial(self.id.clone(), self.config.base_poll_interval_ms))
    }

    /// Read diagnostics snapshot. Non-blocking.
    pub fn get_diagnostics(&self) -> BackgroundDiagnostics {
        BackgroundDiagnostics {
            worker_alive: !self.cancel_token.load(Ordering::Relaxed),
            queue_depth: self.event_queue.len(),
            display_count: 1, // placeholder — wired to DisplayWorkerManager in ServiceManager
            scheduler_interval_ms: self.get_health().current_poll_interval_ms,
            watchdog_running: true, // set by ServiceManager
            future_deadlock_detected: false,
        }
    }

    /// Run the worker loop. Called from a `std::thread` spawned by ServiceManager.
    pub fn run_loop(&self) {
        let mut scheduler = PollingScheduler::new(self.performance_manager.clone());

        self.set_state(WorkerState::Running);

        while !self.cancel_token.load(Ordering::Relaxed) {
            // ── Heartbeat (every loop, even when sleeping/paused) ─────────────
            self.update_heartbeat();

            // ── State check ───────────────────────────────────────────────────
            let state = self.current_state();
            if state == WorkerState::Paused || state == WorkerState::Sleeping {
                thread::sleep(std::time::Duration::from_millis(200));
                continue;
            }
            if state == WorkerState::Stopping {
                break;
            }

            // ── Drain Critical events first ───────────────────────────────────
            let critical_events = self.event_queue.drain_critical();
            for event in &critical_events {
                match event.kind {
                    AdaptiveEventKind::SleepSignal => {
                        self.set_state(WorkerState::Sleeping);
                        continue;
                    }
                    AdaptiveEventKind::WakeFromSleep => {
                        thread::sleep(std::time::Duration::from_millis(
                            self.config.wake_stabilization_delay_ms,
                        ));
                        self.set_state(WorkerState::Running);
                    }
                    AdaptiveEventKind::DisplayRemoved => {
                        // Handled by DisplayWorkerManager; log here only.
                        log::info!("BackgroundWorker: display removed signal received");
                    }
                    _ => {}
                }
                scheduler.on_critical_event();
            }

            // ── Execute one pipeline cycle ────────────────────────────────────
            let pipeline_result = self.execute_cycle(&mut scheduler);

            // ── Update cycle timestamp on completion ──────────────────────────
            if let Ok(mut h) = self.health.lock() {
                h.last_cycle_ms = Some(now_ms());
                if pipeline_result.success {
                    h.last_success_ms = Some(now_ms());
                }
                h.error_count += pipeline_result.error_count;
                h.current_poll_interval_ms = scheduler.current_interval_ms();
            }

            // ── Sleep ─────────────────────────────────────────────────────────
            // Check cancel before sleeping (safe shutdown: finish cycle first).
            if self.cancel_token.load(Ordering::Relaxed) {
                break;
            }
            let sleep_ms = scheduler.next_interval_ms();
            thread::sleep(std::time::Duration::from_millis(sleep_ms));
        }

        self.set_state(WorkerState::Stopped);
        log::info!("BackgroundWorker '{}' stopped cleanly", self.id.0);
    }

    fn execute_cycle(&self, scheduler: &mut PollingScheduler) -> PipelineResult {
        let cycle_start = Instant::now();
        let mut error_count = 0u32;
        let mut changed_brightness = false;
        let mut skipped_reason: Option<String> = None;
        
        let perf_state = self.performance_manager.evaluate_performance_state();

        // ── Step 1: Ambient (non-fatal) ───────────────────────────────────────
        let ambient_start = Instant::now();
        if perf_state.active_policy.pause_ambient {
            // Skipped by policy
        } else {
            // TODO: wire to AmbientManager::get_ambient_light() when sensor is available.
            // Failure here must not stop the cycle — continue with ambient = None.
        }
        let ambient_ms = ambient_start.elapsed().as_millis() as u64;

        // ── Step 2: Screen Analysis (non-fatal) ───────────────────────────────
        let screen_start = Instant::now();
        if perf_state.active_policy.pause_screen_analysis {
            skipped_reason = Some("Screen Analysis paused by Performance Engine (Fullscreen/BatterySaver)".into());
        } else {
            // TODO: wire to ScreenAnalysisManager::analyze_display() when capture is ready.
        }
        let screen_ms = screen_start.elapsed().as_millis() as u64;

        // ── Step 3: Comfort Profile Matching (non-fatal) ──────────────────────
        let comfort_start = Instant::now();
        // TODO: wire to ComfortManager::find_matching_profile().
        let comfort_ms = comfort_start.elapsed().as_millis() as u64;

        // ── Step 4: Visual Comfort Engine ─────────────────────────────────────
        let vce_start = Instant::now();
        // TODO: wire to VisualComfortEngine::calculate_comfort().
        // On Ignore/NoChange — skip brightness update.
        let vce_ms = vce_start.elapsed().as_millis() as u64;

        // ── Step 5: Brightness + Transition (non-fatal) ───────────────────────
        let brightness_start = Instant::now();
        // TODO: wire to AdaptiveBrightnessService::execute_recommendation().
        let brightness_ms = brightness_start.elapsed().as_millis() as u64;
        let transition_ms: u64 = 0;

        // ── Record profiler ───────────────────────────────────────────────────
        let total_ms = cycle_start.elapsed().as_millis() as u64;
        self.profiler.record(PipelineProfile {
            ambient_ms,
            screen_analysis_ms: screen_ms,
            comfort_matching_ms: comfort_ms,
            visual_comfort_ms: vce_ms,
            brightness_ms,
            transition_ms,
            total_ms,
        });

        // ── Notify scheduler ──────────────────────────────────────────────────
        if changed_brightness {
            scheduler.on_change_detected();
        } else {
            scheduler.on_no_change();
        }

        PipelineResult {
            success: error_count == 0,
            reason: if changed_brightness {
                "Brightness adjusted".into()
            } else {
                "No change required".into()
            },
            duration_ms: total_ms,
            changed_brightness,
            skipped_reason: None,
            error_count,
        }
    }

    fn update_heartbeat(&self) {
        if let Ok(mut h) = self.health.lock() {
            h.last_heartbeat_ms = now_ms();
        }
    }

    fn set_state(&self, state: WorkerState) {
        if let Ok(mut h) = self.health.lock() {
            h.current_state = state.clone();
            h.running = matches!(state, WorkerState::Running | WorkerState::Recovering);
        }
    }

    fn current_state(&self) -> WorkerState {
        self.health
            .lock()
            .map(|h| h.current_state.clone())
            .unwrap_or(WorkerState::Stopped)
    }
}

impl Service for BackgroundWorker {
    fn service_id(&self) -> &str {
        &self.id.0
    }

    fn start(&self) -> Result<(), BackgroundError> {
        if self.current_state() == WorkerState::Running {
            return Err(BackgroundError::AlreadyRunning);
        }
        self.cancel_token.store(false, Ordering::Relaxed);
        self.set_state(WorkerState::Initializing);
        log::info!("BackgroundWorker '{}' starting", self.id.0);
        Ok(())
    }

    fn stop(&self) -> Result<(), BackgroundError> {
        if self.current_state() == WorkerState::Stopped {
            return Err(BackgroundError::AlreadyStopped);
        }
        // Signal the loop to stop. It will finish its current cycle before exiting.
        self.set_state(WorkerState::Stopping);
        self.cancel_token.store(true, Ordering::Relaxed);
        log::info!("BackgroundWorker '{}' stop signalled", self.id.0);
        Ok(())
    }

    fn health(&self) -> WorkerHealth {
        self.get_health()
    }
}
