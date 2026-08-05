use crate::ambient::manager::AmbientManager;
use crate::screen_analysis::manager::ScreenAnalysisManager;
use crate::screen_analysis::context::detect_context;
use crate::intelligence::manager::IntelligenceManager;
use crate::intelligence::models::IntelligenceContext;
use crate::brightness::manager::BrightnessManager;
use crate::display::domain::{DisplayInfo, DisplayCapabilities};
use crate::commands::DashboardStatePayload;
use crate::transition::worker::TransitionWorker;
use crate::platform::application::active_window::get_active_application;
use crate::adaptation::policy::{AdaptationPolicy, PolicyContext};
use crate::background::event_log::{EventCategory, LogEvent, SharedEventLog};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

pub struct IntelligencePipeline {
    ambient: Arc<AmbientManager>,
    screen: Arc<ScreenAnalysisManager>,
    intelligence: Arc<IntelligenceManager>,
    brightness: Arc<BrightnessManager>,
    transition: Arc<TransitionWorker>,
    dashboard_state: Arc<Mutex<DashboardStatePayload>>,
    config: Arc<RwLock<crate::configuration::models::AppConfig>>,
    event_log: SharedEventLog,
    running: Arc<AtomicBool>,
}

impl IntelligencePipeline {
    pub fn new(
        ambient: Arc<AmbientManager>,
        screen: Arc<ScreenAnalysisManager>,
        intelligence: Arc<IntelligenceManager>,
        brightness: Arc<BrightnessManager>,
        transition: Arc<TransitionWorker>,
        dashboard_state: Arc<Mutex<DashboardStatePayload>>,
        config: Arc<RwLock<crate::configuration::models::AppConfig>>,
        event_log: SharedEventLog,
        running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            ambient,
            screen,
            intelligence,
            brightness,
            transition,
            dashboard_state,
            config,
            event_log,
            running,
        }
    }

    pub fn start(&self) {
        let ambient = self.ambient.clone();
        let screen = self.screen.clone();
        let intelligence = self.intelligence.clone();
        let brightness = self.brightness.clone();
        let transition = self.transition.clone();
        let dashboard_state = self.dashboard_state.clone();
        let config_lock = self.config.clone();
        let event_log = self.event_log.clone();
        let running = self.running.clone();

        std::thread::spawn(move || {
            let mut last_screen_metrics = None;
            let mut current_context = "Desktop".to_string();
            let mut last_logged_context = String::new();
            let mut last_decision_target: Option<u8> = None;

            // Metrics Trackers
            let mut total_events = 0u64;
            let mut brightness_changes_today = 0u32;
            let manual_overrides_today = 0u32;
            let session_start_time = Instant::now();

            let display = DisplayInfo {
                id: "primary".to_string(),
                name: "Primary".to_string(),
                manufacturer: None,
                model: None,
                width: 1920,
                height: 1080,
                refresh_rate: None,
                is_primary: true,
                capabilities: DisplayCapabilities::default(),
            };

            // Adaptation Policy — owns the "should we adapt now?" decision.
            let mut adaptation_policy = AdaptationPolicy::new();

            // Dynamic polling interval (starts at 2s for fast responsiveness).
            let mut poll_secs: u64 = 1;

            while running.load(Ordering::SeqCst) {
                let cycle_start = Instant::now();
                let config = config_lock.read().unwrap().clone();

                if !config.adaptive.enabled {
                    std::thread::sleep(Duration::from_secs(2));
                    continue;
                }

                total_events += 1;

                // ── 1. Context: Active Application ─────────────────────────────────
                let active_app = get_active_application();

                // ── 2. Ambient Pipeline ─────────────────────────────────────────────
                let ambient_reading = match ambient.get_ambient_light() {
                    Ok(reading) => {
                        if let Ok(mut ds) = dashboard_state.lock() {
                            ds.health.ambient_engine = "Running".into();
                        }
                        Some(reading)
                    }
                    Err(e) => {
                        // Sensor unavailable — continue with reduced confidence, don't stop.
                        if let Ok(mut ds) = dashboard_state.lock() {
                            ds.health.ambient_engine = "Sensor Unavailable".into();
                        }
                        eprintln!("[pipeline] ambient sensor error: {}", e);
                        None
                    }
                };
                let current_lux = ambient_reading.as_ref().map(|r| r.lux).unwrap_or(0.0);
                let current_confidence = ambient_reading.as_ref().map(|r| r.confidence).unwrap_or(0.0);

                // ── 3. Screen Pipeline ──────────────────────────────────────────────
                let screen_result = match screen.analyze_display("primary") {
                    Ok(res) => {
                        if let Ok(mut ds) = dashboard_state.lock() {
                            ds.health.screen_engine = "Running".into();
                        }
                        current_context = detect_context(&res.metrics, last_screen_metrics.as_ref(), &active_app);
                        last_screen_metrics = Some(res.metrics.clone());
                        Some(res)
                    }
                    Err(e) => {
                        if let Ok(mut ds) = dashboard_state.lock() {
                            ds.health.screen_engine = "Capture Unavailable".into();
                        }
                        eprintln!("[pipeline] screen analysis error: {}", e);
                        None
                    }
                };

                let current_luminance = screen_result
                    .as_ref()
                    .map(|r| r.metrics.average_luminance)
                    .unwrap_or(50.0);

                // Log context changes
                if current_context != last_logged_context {
                    if let Ok(mut log) = event_log.lock() {
                        log.push(LogEvent::new(
                            EventCategory::ContextChanged,
                            format!("Context → {}", current_context),
                        ));
                    }
                    last_logged_context = current_context.clone();
                }

                // ── 4. Adaptation Policy ────────────────────────────────────────────
                adaptation_policy.observe(current_lux, current_luminance);
                let policy_ctx = PolicyContext {
                    current_lux,
                    current_luminance,
                    app_context: current_context.clone(),
                    confidence: current_confidence,
                    manual_override_active: {
                        let lock = transition.suspend_until.lock().unwrap();
                        lock.map(|until| Instant::now() < until).unwrap_or(false)
                    },
                    is_fullscreen: false, // TODO: detect via Win32 GetForegroundWindow fullscreen check
                };
                let adaptation_decision = adaptation_policy.should_adapt(&policy_ctx);

                // ── 5. Decision Engine ──────────────────────────────────────────────
                let current_brightness = brightness.get_brightness(&display).unwrap_or(50);

                let int_context = IntelligenceContext {
                    current_time_ms: crate::background::models::now_ms(),
                    comfort_profile: "Adaptive".into(),
                    history_summary: crate::intelligence::models::HistorySummary {
                        total_events: total_events as usize,
                        brightness_changes_today,
                        manual_overrides_today,
                        longest_session_minutes: (session_start_time.elapsed().as_secs() / 60) as u32,
                        average_ambient_lux: current_lux,
                    },
                    current_ambient_lux: current_lux,
                    current_screen_luminance: current_luminance,
                    worker_running: true,
                    performance_policy: "Balanced".into(),
                    active_application: active_app.clone(),
                    active_display_id: "primary".into(),
                    confidence_score: current_confidence,
                };

                let payload = intelligence.generate_payload(
                    &int_context,
                    current_brightness,
                    config.brightness.comfort_profile.clone(),
                );

                // ── 6. Transition Gate ──────────────────────────────────────────────
                if adaptation_decision.is_adapt() {
                    if let Some(target) = payload.current_decision.target_brightness {
                        let prev_target = last_decision_target.unwrap_or(current_brightness);
                        if (target as i32 - prev_target as i32).abs() >= 3 {
                            let previous_target = transition.target_brightness.load(Ordering::SeqCst);
                            if target != previous_target {
                                // Log the brightness change event with Reason
                                if let Ok(mut log) = event_log.lock() {
                                    let reason = payload.current_decision.reason.clone();
                                    log.push(
                                        LogEvent::new(
                                            EventCategory::BrightnessChanged,
                                            &format!("Reason: {}", reason),
                                        )
                                        .with_values(
                                            format!("{}%", current_brightness),
                                            format!("{}%", target),
                                        ),
                                    );
                                }
                                transition.set_target(target);
                                brightness_changes_today += 1;
                                last_decision_target = Some(target);
                            }
                        }
                    }
                } else {
                    // Log when adaptation was skipped (for debugging).
                    // Only log once per skip reason to avoid flooding.
                    if let Ok(mut log) = event_log.lock() {
                        let reason = adaptation_decision.reason().to_string();
                        // Only push if different from last event
                        let last_skip = log.get_recent().first()
                            .filter(|e| e.category == EventCategory::AdaptationSkipped)
                            .map(|e| e.description.clone());
                        if last_skip.as_deref() != Some(&reason) {
                            log.push(LogEvent::new(EventCategory::AdaptationSkipped, reason));
                        }
                    }
                }

                // ── 7. Update Dashboard State ───────────────────────────────────────
                let cycle_ms = cycle_start.elapsed().as_millis() as u64;
                if let Ok(mut state) = dashboard_state.lock() {
                    state.ambient.lux = Some(current_lux);
                    state.ambient.confidence = Some(current_confidence);
                    state.ambient.source = ambient_reading
                        .as_ref()
                        .map(|r| r.source_id.clone())
                        .unwrap_or("Manual".into());

                    if let Some(res) = &screen_result {
                        state.screen.average_luminance = Some(res.metrics.average_luminance);
                        state.screen.peak_luminance = Some(res.metrics.peak_luminance);
                        state.screen.current_analysis_time_ms = Some(res.analysis_duration_ms);
                        state.screen.context = Some(current_context.clone());
                    }

                    state.brightness.current = Some(current_brightness);
                    state.brightness.target = payload.current_decision.target_brightness;
                    state.intelligence = payload;
                    state.comfort.mode = current_context.clone();
                    state.performance.pipeline_duration_ms = Some(cycle_ms);
                    state.performance.current_poll_interval_ms = Some(poll_secs * 1000);
                }

                // ── 8. Performance Budget ───────────────────────────────────────────
                // If cycle took longer than 800ms, reduce frequency to protect CPU.
                if cycle_ms > 800 {
                    poll_secs = (poll_secs + 1).min(10);
                } else if cycle_ms < 200 && poll_secs > 1 {
                    poll_secs = (poll_secs - 1).max(1);
                }

                // Adaptive sleep based on context (sub-second for active work).
                let context_sleep_ms: u64 = match current_context.as_str() {
                    "Video"   => 2000,  // Video: slow poll is fine
                    "Gaming"  => 5000,  // Gaming: minimal polling
                    "Reading" => 1000,  // Reading: moderate
                    "Coding"  => 500,   // Coding: fast — user switches tabs frequently
                    _         => 500,   // Default/Desktop: fast
                };

                let elapsed = cycle_start.elapsed();
                let sleep_dur = Duration::from_millis(context_sleep_ms).saturating_sub(elapsed);
                std::thread::sleep(sleep_dur);
            }
        });
    }
}
