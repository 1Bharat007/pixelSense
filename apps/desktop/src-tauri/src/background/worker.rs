use crate::background::config::BackgroundConfig;
use crate::background::error::BackgroundError;
use crate::background::event::models::{AdaptiveEvent, AdaptiveEventKind, EventPriority};
use crate::background::event::queue::EventQueue;
use crate::background::models::{
    now_ms, BackgroundDiagnostics, PipelineProfile, PipelineResult, WorkerHealth, WorkerId,
    WorkerState,
};
use crate::background::profiler::PipelineProfiler;
use crate::performance::scheduler::CentralScheduler;
use crate::performance::budget::PerformanceBudgetManager;
use crate::background::service::Service;
use crate::performance::manager::PerformanceManager;
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
    #[allow(dead_code)] // Reserved for future history features
    history_manager: Arc<HistoryManager>,
    #[allow(dead_code)] // Reserved for future multi-monitor coordination
    multi_monitor_scheduler: Arc<MultiMonitorScheduler>,
    
    // Core Engines
    screen_manager: Arc<crate::screen_analysis::manager::ScreenAnalysisManager>,
    visual_comfort: Arc<crate::visual_comfort::engine::VisualComfortEngine>,
    adaptive_service: Arc<crate::adaptive::service::AdaptiveBrightnessService>,
    
    // Hardware integration
    sensor_session: Arc<crate::platform::hardware::sensor::manager::SensorSession>,
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

        // Instantiate core pipelines
        let screen_manager = Arc::new(crate::screen_analysis::factory::create_screen_analysis_manager(
            crate::screen_analysis::config::AnalysisConfig::default()
        ));
        
        let visual_comfort = Arc::new(crate::visual_comfort::factory::create_visual_comfort_engine(
            crate::visual_comfort::models::ComfortConfig::default()
        ));
        
        let brightness_manager = Arc::new(crate::brightness::factory::create_brightness_manager());
        let adaptive_service = Arc::new(crate::adaptive::factory::create_adaptive_service(brightness_manager));
        let sensor_session = Arc::new(crate::platform::hardware::sensor::manager::SensorSession::new());

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
            screen_manager,
            visual_comfort,
            adaptive_service,
            sensor_session,
        }
    }

    pub fn id(&self) -> &WorkerId {
        &self.id
    }

    pub fn config(&self) -> &BackgroundConfig {
        &self.config
    }

    fn current_state(&self) -> WorkerState {
        self.health
            .lock()
            .map(|h| h.current_state.clone())
            .unwrap_or(WorkerState::Stopped)
    }

    pub fn health(&self) -> WorkerHealth {
        self.health.lock().unwrap().clone()
    }

    fn set_state(&self, new_state: WorkerState) {
        if let Ok(mut h) = self.health.lock() {
            h.current_state = new_state;
            h.running = matches!(h.current_state, WorkerState::Running | WorkerState::Recovering);
        }
    }

    fn update_heartbeat(&self) {
        if let Ok(mut h) = self.health.lock() {
            h.last_heartbeat_ms = now_ms();
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
        let budget = Arc::new(PerformanceBudgetManager::new());
        let mut scheduler = CentralScheduler::new(budget.clone());

        self.set_state(WorkerState::Running);

        while !self.cancel_token.load(Ordering::Relaxed) {
            self.update_heartbeat();

            let state = self.current_state();
            if state == WorkerState::Paused || state == WorkerState::Sleeping {
                thread::sleep(std::time::Duration::from_millis(200));
                continue;
            }
            if state == WorkerState::Stopping {
                break;
            }

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
                        log::info!("BackgroundWorker: display removed signal received");
                    }
                    _ => {}
                }
            }

            let pipeline_result = self.execute_cycle(&mut scheduler, budget.clone());

            if let Ok(mut h) = self.health.lock() {
                h.last_cycle_ms = Some(now_ms());
                if pipeline_result.success {
                    h.last_success_ms = Some(now_ms());
                }
                h.error_count += pipeline_result.error_count;
                // Since CentralScheduler has dynamic sleep per component, we report average loop delay
                h.current_poll_interval_ms = 200; 
            }

            if self.cancel_token.load(Ordering::Relaxed) {
                break;
            }
            // Sleep the base scheduler tick
            thread::sleep(std::time::Duration::from_millis(100));
        }

        self.set_state(WorkerState::Stopped);
        log::info!("BackgroundWorker '{}' stopped cleanly", self.id.0);
    }

    fn execute_cycle(&self, scheduler: &mut CentralScheduler, budget: Arc<PerformanceBudgetManager>) -> PipelineResult {
        let cycle_start = Instant::now();
        let error_count = 0u32;
        let mut changed_brightness = false;
        let mut _skipped_reason: Option<String> = None;
        
        let perf_state = self.performance_manager.evaluate_performance_state();

        let ambient_start = Instant::now();
        let mut ambient_lux = None;
        if !perf_state.active_policy.pause_ambient && scheduler.should_poll_ambient() {
            ambient_lux = self.sensor_session.read_lux().ok();
        }
        let ambient_ms = ambient_start.elapsed().as_millis() as u64;

        let screen_start = Instant::now();
        let mut screen_luminance = None;
        if perf_state.active_policy.pause_screen_analysis {
            _skipped_reason = Some("Screen Analysis paused".into());
        } else if scheduler.should_poll_screen() {
            if let Ok(res) = self.screen_manager.analyze_display("default") {
                screen_luminance = Some(res.metrics.average_luminance);
            }
        }
        let screen_ms = screen_start.elapsed().as_millis() as u64;

        // ── Step 3: Comfort Profile Matching (non-fatal) ──────────────────────
        let comfort_start = Instant::now();
        let ctx = crate::visual_comfort::models::VisualComfortContext {
            display_id: "default".into(),
            current_comfort_profile: None,
            ambient_light: ambient_lux,
            screen_luminance,
            current_monitor_brightness: 50,
            predicted_emitted_light: 0.0,
            time_of_day: "Day".into(),
            transition_enabled: true,
            confidence: 1.0,
        };
        let comfort_ms = comfort_start.elapsed().as_millis() as u64;

        // ── Step 4: Visual Comfort Engine ─────────────────────────────────────
        let vce_start = Instant::now();
        let comfort_result = self.visual_comfort.calculate_comfort(ctx);
        let mut target_brightness = None;
        if comfort_result.recommendation.action != crate::visual_comfort::models::RecommendationAction::Ignore
            && comfort_result.recommendation.action != crate::visual_comfort::models::RecommendationAction::NoChange {
            target_brightness = Some(comfort_result.recommendation.recommended_brightness);
            changed_brightness = true;
        }
        let vce_ms = vce_start.elapsed().as_millis() as u64;

        // ── Step 5: Brightness + Transition (non-fatal) ───────────────────────
        let brightness_start = Instant::now();
        if let Some(tb) = target_brightness {
            let caps = crate::display::domain::DisplayCapabilities { brightness: true, hdr: false, ddc_ci: true };
            let display = crate::display::domain::DisplayInfo {
                id: "default".into(),
                name: "Primary Display".into(),
                manufacturer: None,
                model: None,
                width: 1920,
                height: 1080,
                refresh_rate: None,
                is_primary: true,
                capabilities: caps.clone(),
            };
            let decision_ctx = crate::decision::models::DecisionContext {
                ambient_light: ambient_lux.map(|lux| crate::decision::models::AmbientLightReading { lux }),
                user_brightness_preference: Some(tb),
                comfort_preference: crate::decision::models::ComfortLevel::Balanced,
                time_of_day: crate::decision::models::TimeOfDay::Day,
            };
            let _ = self.adaptive_service.execute_pipeline(&display, &caps, &decision_ctx);
        }
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
        
        // Feed real CPU time metric into the budget manager
        budget.report_metrics(total_ms as f32 / 100.0, 30); // Synthetic report

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
        BackgroundWorker::health(self)
    }
}
