use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use crate::commands::{DashboardStatePayload, ComfortStatePayload, AmbientStatePayload, ScreenStatePayload, BrightnessStatePayload, PerformanceStatePayload, EngineHealthPayload};
use crate::intelligence::manager::IntelligenceManager;
use crate::intelligence::models::IntelligenceContext;
use crate::intelligence::models::HistorySummary;
use crate::platform::hardware::sensor::manager::SensorSession;
use crate::platform::hardware::wmi::manager::WmiBrightnessManager;
use std::collections::VecDeque;
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
}

impl ServiceRegistry {
    pub fn new(initial_config: crate::configuration::models::AppConfig) -> Self {
        let state = DashboardStatePayload {
            comfort: ComfortStatePayload {
                status: "Waiting for Sensor".into(),
                recommendation: "Cannot determine comfort without sensor data.".into(),
                confidence: None,
                active_profile: "Adaptive".into(),
                mode: "Adaptive".into(),
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
            }),
        };

        Self {
            config: Arc::new(RwLock::new(initial_config)),
            dashboard_state: Arc::new(Mutex::new(state)),
            worker_running: Arc::new(AtomicBool::new(false)),
            watchdog_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start_hardware_worker(&self) {
        if self.worker_running.swap(true, Ordering::SeqCst) {
            // Worker is already running. Rule: Single-Instance Engine.
            return;
        }

        let state_clone = self.dashboard_state.clone();
        let running_clone = self.worker_running.clone();
        let config_clone = self.config.clone();
        
        // Shared state between Decision Engine and Transition Engine
        let global_target_brightness = Arc::new(AtomicI32::new(-1));
        
        let target_clone_for_transition = global_target_brightness.clone();
        let running_clone_for_transition = self.worker_running.clone();
        let config_clone_for_transition = self.config.clone();
        let state_clone_for_transition = self.dashboard_state.clone();
        
        // 1. Transition Engine (Runs Asynchronously at High Frequency)
        std::thread::spawn(move || {
            let display_provider: Box<dyn DisplayProvider> = Box::new(WmiBrightnessManager::new());
            
            loop {
                if !running_clone_for_transition.load(Ordering::SeqCst) { break; }
                
                let target = target_clone_for_transition.load(Ordering::SeqCst);
                if target >= 0 {
                    let mut current = display_provider.brightness().unwrap_or(target as u8) as i32;
                    if current != target {
                        if let Ok(mut state) = state_clone_for_transition.lock() {
                            state.brightness.transition_status = "Active".into();
                        }
                        
                        let step = if target > current { 2 } else { -2 };
                        current += step;
                        if step > 0 && current > target { current = target; }
                        if step < 0 && current < target { current = target; }
                        
                        let _ = display_provider.set_brightness(current as u8);
                    } else {
                        if let Ok(mut state) = state_clone_for_transition.lock() {
                            state.brightness.transition_status = "Waiting".into();
                        }
                    }
                }
                
                let transition_interval = config_clone_for_transition
                    .read().unwrap()
                    .adaptive.transition_interval_ms.unwrap_or(50);
                std::thread::sleep(std::time::Duration::from_millis(transition_interval));
            }
        });

        // 2. Decision Engine & Sensor Polling (Runs at Low Frequency)
        std::thread::spawn(move || {
            let ambient_provider: Box<dyn AmbientProvider> = Box::new(SensorSession::new());
            let display_provider: Box<dyn DisplayProvider> = Box::new(WmiBrightnessManager::new());
            let mut lux_history: VecDeque<f32> = VecDeque::new();
            
            loop {
                // Read live configuration from in-memory lock for zero-I/O power efficiency
                let config = config_clone.read().unwrap().clone();
                
                if !config.adaptive.enabled {
                    if let Ok(mut state) = state_clone.lock() {
                        state.comfort.status = "Protection Paused".into();
                        state.brightness.transition_status = "Paused".into();
                        state.ambient.health = "Paused".into();
                    }
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    
                    if !running_clone.load(Ordering::SeqCst) {
                        break;
                    }
                    continue; // Skip hardware polling to save battery
                }

                let lux_result = ambient_provider.current_lux();
                let current_lux = lux_result.ok().flatten();
                
                let mut target_brightness = None;
                if let Some(l) = current_lux {
                    // Update Moving Average Filter (store up to 15 samples, ~15 seconds)
                    lux_history.push_back(l);
                    if lux_history.len() > 15 {
                        lux_history.pop_front();
                    }
                    
                    let avg_lux: f32 = lux_history.iter().sum::<f32>() / (lux_history.len() as f32);
                    
                    let baseline_lux = config.brightness.reference_ambient_lux.unwrap_or(100.0);
                    let delta_lux = avg_lux - baseline_lux;
                    let ref_bright = config.brightness.reference_brightness.unwrap_or(50) as f32;
                    let mut b = (ref_bright + (delta_lux * 0.1)) as u8;
                    if b < 10 { b = 10; } // hard floor 10%
                    if b > 100 { b = 100; }
                    
                    target_brightness = Some(b);
                }
                
                let current_brightness = display_provider.brightness().ok();
                if let (Some(target), Some(current)) = (target_brightness, current_brightness) {
                    if (current as i32 - target as i32).abs() > 3 {
                        global_target_brightness.store(target as i32, Ordering::SeqCst);
                        if let Ok(mut state) = state_clone.lock() {
                            state.brightness.target = Some(target);
                        }
                    }
                } else if let Some(target) = target_brightness {
                    global_target_brightness.store(target as i32, Ordering::SeqCst);
                }
                
                if let Ok(mut state) = state_clone.lock() {
                    state.ambient.lux = current_lux;
                    if current_lux.is_some() {
                        state.ambient.health = "Active".into();
                        state.ambient.source = "Windows Sensor API".into();
                        state.comfort.status = "Protection Active".into();
                    } else {
                        state.ambient.health = "Unavailable".into();
                        state.ambient.source = "None".into();
                        state.comfort.status = "Waiting for Sensor".into();
                    }
                    
                    state.brightness.target = target_brightness;
                    state.brightness.current = current_brightness;
                    if target_brightness.is_some() && current_brightness != target_brightness {
                        state.brightness.transition_status = "Active".into();
                    } else if target_brightness.is_some() {
                        state.brightness.transition_status = "Waiting".into();
                    } else {
                        state.brightness.transition_status = "Unavailable".into();
                    }
                }
                
                let poll_interval = config.adaptive.poll_interval_ms.unwrap_or(1000);
                std::thread::sleep(std::time::Duration::from_millis(poll_interval));
                
                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
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
