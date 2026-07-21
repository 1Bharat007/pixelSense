use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::commands::{DashboardStatePayload, ComfortStatePayload, AmbientStatePayload, ScreenStatePayload, BrightnessStatePayload, PerformanceStatePayload, EngineHealthPayload};
use crate::intelligence::manager::IntelligenceManager;
use crate::intelligence::models::IntelligenceContext;
use crate::intelligence::models::HistorySummary;
use crate::platform::hardware::sensor::manager::SensorSession;
use crate::platform::hardware::wmi::manager::WmiBrightnessManager;
use crate::background::event_log::{SharedEventLog, new_shared_event_log};
use crate::brightness::memory::AppBrightnessMemory;
use sysinfo::System;

pub trait AmbientProvider: Send + Sync {
    fn current_lux(&self) -> Result<Option<f32>, String>;
}

pub trait DisplayProvider: Send + Sync {
    fn brightness(&self) -> Result<u8, String>;
    fn set_brightness(&self, value: u8) -> Result<(), String>;
}

impl AmbientProvider for SensorSession {
    fn current_lux(&self) -> Result<Option<f32>, String> {
        self.read_lux().map(Some).map_err(|e| e.to_string())
    }
}

impl DisplayProvider for WmiBrightnessManager {
    fn brightness(&self) -> Result<u8, String> {
        self.get_brightness().map_err(|e| e.to_string())
    }
    fn set_brightness(&self, value: u8) -> Result<(), String> {
        self.set_brightness(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone)]
pub struct ServiceRegistry {
    pub config: Arc<RwLock<crate::configuration::models::AppConfig>>,
    pub dashboard_state: Arc<Mutex<DashboardStatePayload>>,
    pub worker_running: Arc<AtomicBool>,
    pub watchdog_running: Arc<AtomicBool>,
    pub transition_worker: Arc<RwLock<Option<Arc<crate::transition::worker::TransitionWorker>>>>,
    pub brightness_manager: Arc<crate::brightness::manager::BrightnessManager>,
    pub event_log: SharedEventLog,
    pub brightness_memory: Arc<Mutex<AppBrightnessMemory>>,
}

impl ServiceRegistry {
    pub fn new(initial_config: crate::configuration::models::AppConfig) -> Self {
        let mut state = DashboardStatePayload {
            comfort: ComfortStatePayload {
                status: "Waiting for Sensor".into(),
                recommendation: "Cannot determine comfort without sensor data.".into(),
                confidence: None,
                active_profile: "Adaptive".into(),
                mode: "Adaptive".into(),
                explanation: None,
            },
            ambient: AmbientStatePayload {
                lux: None,
                environment: "Unknown".into(),
                health: "Unavailable".into(),
                confidence: None,
                source: "None".into(),
            },
            screen: ScreenStatePayload {
                average_luminance: None,
                peak_luminance: None,
                visual_complexity: None,
                current_analysis_time_ms: None,
                context: None,
            },
            brightness: BrightnessStatePayload {
                current: None,
                target: None,
                transition_status: "Unavailable".into(),
                transition_progress: None,
                eye_comfort_score: None,
            },
            performance: PerformanceStatePayload {
                cpu_usage_pct: None,
                ram_usage_mb: None,
                current_poll_interval_ms: None,
                battery_mode: "Unknown".into(),
                power_state: "Unknown".into(),
                pipeline_duration_ms: None,
            },
            health: EngineHealthPayload {
                background_worker: "Idle".into(),
                watchdog: "Idle".into(),
                ambient_engine: "Waiting".into(),
                screen_engine: "Waiting".into(),
                comfort_engine: "Waiting".into(),
                transition_engine: "Waiting".into(),
            },
            intelligence: IntelligenceManager::new().generate_payload(&IntelligenceContext {
                current_time_ms: 0,
                comfort_profile: "Adaptive".into(),
                history_summary: HistorySummary {
                    total_events: 0,
                    brightness_changes_today: 0,
                    manual_overrides_today: 0,
                    longest_session_minutes: 0,
                    average_ambient_lux: 0.0,
                },
                current_ambient_lux: 0.0,
                current_screen_luminance: 0.0,
                worker_running: false,
                performance_policy: "Balanced".into(),
                active_application: "Unknown".into(),
                active_display_id: "Unknown".into(),
                confidence_score: 0.0,
            }, 50, None),
        };

        use crate::brightness::providers::native::NativeBrightnessProvider;
        use crate::brightness::manager::BrightnessManager;
        use crate::display::domain::DisplayInfo;
        let provider = Box::new(NativeBrightnessProvider::new());
        let brightness_manager = Arc::new(BrightnessManager::new(provider));

        let primary = DisplayInfo {
            id: "primary".to_string(),
            name: "Primary".to_string(),
            manufacturer: None,
            model: None,
            width: 1920,
            height: 1080,
            refresh_rate: None,
            is_primary: true,
            capabilities: Default::default(),
        };

        if let Ok(b) = brightness_manager.get_brightness(&primary) {
            state.brightness.current = Some(b);
            state.brightness.transition_status = "Stable".into();
        }

        Self {
            config: Arc::new(RwLock::new(initial_config)),
            dashboard_state: Arc::new(Mutex::new(state)),
            worker_running: Arc::new(AtomicBool::new(false)),
            watchdog_running: Arc::new(AtomicBool::new(false)),
            transition_worker: Arc::new(RwLock::new(None)),
            brightness_manager,
            event_log: new_shared_event_log(),
            brightness_memory: Arc::new(Mutex::new(AppBrightnessMemory::new())),
        }
    }

    pub fn start_hardware_worker(&self) {
        if self.worker_running.swap(true, Ordering::SeqCst) {
            return;
        }

        // Setup Ambient Pipeline
        use crate::ambient::manager::AmbientManager;
        use crate::ambient::registry::SensorRegistry;
        use crate::ambient::config::AmbientConfig;
        use crate::ambient::calibration::linear::LinearCalibration;
        use crate::ambient::smoothing::BasicSmoothingStrategy;
        use crate::platform::hardware::sensor::provider::NativeSensorProvider;
        
        let mut ambient_registry = SensorRegistry::new();
        ambient_registry.register(std::sync::Arc::new(NativeSensorProvider::new()));
        let ambient = Arc::new(AmbientManager::new(
            AmbientConfig::default(),
            ambient_registry,
            Box::new(LinearCalibration::new(1000.0)),
            Box::new(BasicSmoothingStrategy::new(2))
        ));

        // Setup Screen Pipeline
        use crate::screen_analysis::manager::ScreenAnalysisManager;
        use crate::screen_analysis::config::AnalysisConfig;
        use crate::screen_analysis::providers::windows_provider::WindowsScreenProvider;
        
        let screen = Arc::new(ScreenAnalysisManager::new(
            AnalysisConfig::default(),
            Box::new(WindowsScreenProvider::new())
        ));

        // Setup Decision & Comfort Pipeline
        let intelligence = Arc::new(IntelligenceManager::new());
        
        // Setup Brightness Pipeline
        let brightness = self.brightness_manager.clone();

        // Setup Transition Engine
        use crate::transition::worker::TransitionWorker;
        let transition_worker = Arc::new(TransitionWorker::new(
            brightness.clone(),
            self.config.clone(),
            self.worker_running.clone(),
            self.dashboard_state.clone(),
        ));
        
        if let Ok(mut lock) = self.transition_worker.write() {
            *lock = Some(transition_worker.clone());
        }
        
        transition_worker.start();

        // Assemble Intelligence Pipeline
        use crate::intelligence::pipeline::IntelligencePipeline;
        let pipeline = IntelligencePipeline::new(
            ambient,
            screen,
            intelligence,
            brightness,
            transition_worker,
            self.dashboard_state.clone(),
            self.config.clone(),
            self.event_log.clone(),
            self.worker_running.clone(),
        );

        pipeline.start();
    }

    pub fn start_watchdog(&self) {
        if self.watchdog_running.swap(true, Ordering::SeqCst) {
            return;
        }

        let state_clone = self.dashboard_state.clone();
        let config_clone = self.config.clone();
        std::thread::spawn(move || {
            let mut sys = System::new_all();
            let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
            let mut critical_strikes = 0;
            
            loop {
                sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
                if let Some(process) = sys.process(pid) {
                    let memory_usage_mb = process.memory() as f32 / 1024.0 / 1024.0;
                    // Note: sysinfo cpu_usage is total across all cores.
                    // We can divide by sys.cpus().len() if we want per-core average.
                    
                    if let Ok(mut state) = state_clone.lock() {
                        state.performance.ram_usage_mb = Some(memory_usage_mb);
                        state.health.watchdog = "Active".into();
                    }

                    // Self-Preservation Rule: Graceful shutdown if RAM > 250MB for 3 consecutive polls
                    if memory_usage_mb > 250.0 {
                        critical_strikes += 1;
                        if critical_strikes >= 3 {
                            eprintln!("CRITICAL: PixelSense memory leak detected ({} MB for 15s). Gracefully disabling engine.", memory_usage_mb);
                            if let Ok(mut cfg) = config_clone.write() {
                                cfg.adaptive.enabled = false;
                            }
                            if let Ok(mut state) = state_clone.lock() {
                                state.health.ambient_engine = "Error".into();
                                state.comfort.status = "Engine Error".into();
                            }
                            // Do not exit, just let the disabled engine sleep
                        }
                    } else {
                        critical_strikes = 0; // Reset on healthy memory
                    }
                }
                
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        });
    }
}
