use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use crate::brightness::manager::BrightnessManager;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use crate::configuration::models::AppConfig;
use crate::commands::DashboardStatePayload;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionState {
    Idle,
    Pending,
    Transitioning,
    Settling,
}

impl TransitionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Pending => "Pending",
            Self::Transitioning => "Adjusting",
            Self::Settling => "Settling",
        }
    }
}

pub struct TransitionWorker {
    pub target_brightness: Arc<AtomicU8>,
    pub suspend_until: Arc<Mutex<Option<Instant>>>,
    brightness_manager: Arc<BrightnessManager>,
    config: Arc<RwLock<AppConfig>>,
    running: Arc<AtomicBool>,
    dashboard_state: Arc<Mutex<DashboardStatePayload>>,
    /// Timestamp of last completed transition (for cooldown guard).
    last_transition_completed: Arc<Mutex<Option<Instant>>>,
}

impl TransitionWorker {
    pub fn new(
        brightness_manager: Arc<BrightnessManager>,
        config: Arc<RwLock<AppConfig>>,
        running: Arc<AtomicBool>,
        dashboard_state: Arc<Mutex<DashboardStatePayload>>,
    ) -> Self {
        Self {
            target_brightness: Arc::new(AtomicU8::new(0)),
            suspend_until: Arc::new(Mutex::new(None)),
            brightness_manager,
            config,
            running,
            dashboard_state,
            last_transition_completed: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_target(&self, target: u8) {
        self.target_brightness.store(target, Ordering::SeqCst);
    }

    pub fn suspend_automation(&self) {
        let suspend_ms = {
            let config = self.config.read().unwrap();
            config.brightness.manual_override_suspend_ms
        };
        let mut lock = self.suspend_until.lock().unwrap();
        *lock = Some(Instant::now() + Duration::from_millis(suspend_ms));
    }

    fn ease(t: f32, curve: &str) -> f32 {
        match curve {
            "Linear" => t,
            "EaseInOut" => {
                let sq = t * t;
                sq / (2.0 * (sq - t) + 1.0)
            },
            "Smooth" => t * t * (3.0 - 2.0 * t), // Smoothstep
            "Natural" => 1.0 - (1.0 - t).powi(3), // EaseOutCubic (matches eye perception better)
            _ => t * t * (3.0 - 2.0 * t), // Default smooth
        }
    }

    pub fn start(&self) {
        let target_brightness = self.target_brightness.clone();
        let suspend_until = self.suspend_until.clone();
        let brightness_manager = self.brightness_manager.clone();
        let config_lock = self.config.clone();
        let running = self.running.clone();
        let dashboard_state = self.dashboard_state.clone();
        let last_transition_completed = self.last_transition_completed.clone();

        thread::spawn(move || {
            let display = DisplayInfo {
                id: "primary".to_string(),
                name: "Primary".to_string(),
                manufacturer: None,
                model: None,
                width: 1920,
                height: 1080,
                refresh_rate: None,
                is_primary: true,
                capabilities: DisplayCapabilities { brightness: true, hdr: false, ddc_ci: true },
            };
            let capabilities = DisplayCapabilities { brightness: true, hdr: false, ddc_ci: true };
            
            let mut state = TransitionState::Idle;
            let mut current_brightness: f32 = brightness_manager.get_brightness(&display).unwrap_or(50) as f32;
            target_brightness.store(current_brightness as u8, Ordering::SeqCst);
            
            let mut transition_start_time = Instant::now();
            let mut transition_duration = Duration::from_millis(500);
            let mut start_brightness = current_brightness;
            let mut end_brightness = current_brightness;
            
            while running.load(Ordering::SeqCst) {
                let (enabled, hysteresis_pct, min_b, max_b, curve, dur_ms) = {
                    let c = config_lock.read().unwrap();
                    (
                        c.transition.enabled,
                        c.transition.hysteresis_pct,
                        c.brightness.comfort_profile.as_ref().map(|p| p.min_brightness).unwrap_or(0),
                        c.brightness.comfort_profile.as_ref().map(|p| p.max_brightness).unwrap_or(100),
                        c.transition.easing_curve.clone(),
                        c.transition.duration_ms
                    )
                };

                if !enabled {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }

                // Check suspension
                {
                    let mut lock = suspend_until.lock().unwrap();
                    if let Some(until) = *lock {
                        if Instant::now() < until {
                            if let Ok(mut ds) = dashboard_state.lock() {
                                ds.brightness.transition_status = "Suspended (Manual)".into();
                            }
                            thread::sleep(Duration::from_millis(100));
                            
                            // Re-sync current brightness as user might be changing it
                            if let Ok(b) = brightness_manager.get_brightness(&display) {
                                current_brightness = b as f32;
                                target_brightness.store(b, Ordering::SeqCst);
                            }
                            continue;
                        } else {
                            *lock = None;
                        }
                    }
                }

                let target_raw = target_brightness.load(Ordering::SeqCst);
                let target_clamped = target_raw.clamp(min_b, max_b);

                // State Machine
                match state {
                    TransitionState::Idle => {
                        let diff = (current_brightness - target_clamped as f32).abs();
                        if diff >= hysteresis_pct as f32 {
                            state = TransitionState::Pending;
                        }
                    },
                    TransitionState::Pending => {
                        // Cooldown guard: after a transition completes, wait before starting another.
                        // This prevents rapid oscillation between two states.
                        let cooldown_ms = {
                            let c = config_lock.read().unwrap();
                            c.transition.cooldown_ms
                        };
                        let in_cooldown = {
                            let lock = last_transition_completed.lock().unwrap();
                            lock.map(|t| t.elapsed() < Duration::from_millis(cooldown_ms))
                                .unwrap_or(false)
                        };

                        if in_cooldown {
                            // Stay in Pending until cooldown expires — don't start a new transition.
                            if let Ok(mut ds) = dashboard_state.lock() {
                                ds.brightness.transition_status = "Cooldown".into();
                            }
                        } else {
                            start_brightness = current_brightness;
                            end_brightness = target_clamped as f32;
                            // Adaptive duration: bigger jumps take longer (smoother visual).
                            let magnitude = (end_brightness - start_brightness).abs();
                            let adaptive_ms = if magnitude > 20.0 {
                                (dur_ms as f32 * 1.5) as u64  // big jump → slower
                            } else if magnitude < 8.0 {
                                (dur_ms as f32 * 0.6) as u64  // small correction → quicker
                            } else {
                                dur_ms
                            };
                            transition_duration = Duration::from_millis(adaptive_ms.max(200));
                            transition_start_time = Instant::now();
                            state = TransitionState::Transitioning;
                        }
                    },
                    TransitionState::Transitioning => {
                        let elapsed = transition_start_time.elapsed();
                        if elapsed >= transition_duration {
                            current_brightness = end_brightness;
                            if let Err(e) = brightness_manager.set_brightness(&display, &capabilities, current_brightness as i32) {
                                log::warn!("Failed to set final transition brightness: {}", e);
                            }
                            state = TransitionState::Settling;
                        } else {
                            let t = elapsed.as_secs_f32() / transition_duration.as_secs_f32();
                            let eased_t = Self::ease(t, &curve);
                            let new_brightness = start_brightness + (end_brightness - start_brightness) * eased_t;
                            
                            let old_rounded = current_brightness.round() as i32;
                            let new_rounded = new_brightness.round() as i32;
                            
                            if old_rounded != new_rounded {
                                if let Err(e) = brightness_manager.set_brightness(&display, &capabilities, new_rounded) {
                                    log::warn!("Failed to set intermediate transition brightness: {}", e);
                                }
                            }
                            current_brightness = new_brightness;
                        }
                    },
                    TransitionState::Settling => {
                        // IPC Verification: Read back hardware brightness
                        std::thread::sleep(Duration::from_millis(50));
                        let actual = match brightness_manager.get_brightness(&display) {
                            Ok(v) => v,
                            Err(e) => {
                                log::warn!("Failed to read brightness in Settling state: {}", e);
                                0
                            }
                        };
                        let success = (actual as i32 - end_brightness as i32).abs() <= 5;
                        
                        let ambient_lux = if let Ok(ds) = dashboard_state.lock() {
                            ds.ambient.lux.unwrap_or(0.0)
                        } else {
                            0.0
                        };

                        if success {
                            println!("\n=== Automatic Engine Adjustment ===");
                            println!("Ambient:\n  {} lux", ambient_lux);
                            println!("Current brightness:\n  {}%", start_brightness.round() as u8);
                            println!("Target:\n  {}%", end_brightness.round() as u8);
                            println!("Provider:\n  WMI/DDC executed");
                            println!("Read-back:\n  {}%\n", actual);
                        }

                        if let Ok(mut ds) = dashboard_state.lock() {
                            if !success {
                                ds.health.transition_engine = "Hardware Error".into();
                            } else {
                                ds.health.transition_engine = "Running".into();
                            }
                        }
                        
                        // Record that this transition completed, to start the cooldown timer.
                        if let Ok(mut lock) = last_transition_completed.lock() {
                            *lock = Some(Instant::now());
                        }
                        state = TransitionState::Idle;
                    }
                }

                if let Ok(mut ds) = dashboard_state.lock() {
                    ds.brightness.transition_status = state.as_str().into();
                    if state == TransitionState::Transitioning {
                        let elapsed = transition_start_time.elapsed().as_secs_f32();
                        let total = transition_duration.as_secs_f32();
                        ds.brightness.transition_progress = Some((elapsed / total).clamp(0.0, 1.0));
                        ds.brightness.current = Some(current_brightness.round() as u8);
                    } else {
                        ds.brightness.transition_progress = None;
                        ds.brightness.current = Some(current_brightness.round() as u8);
                    }
                }

                // Smooth ~30FPS updates during transition, sleep otherwise
                if state == TransitionState::Transitioning {
                    thread::sleep(Duration::from_millis(32));
                } else {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });
    }
}
