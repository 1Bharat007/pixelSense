use crate::performance::config::PerformanceConfig;
use crate::performance::models::{PerformanceDiagnostics, PerformanceState, PowerState};
use crate::performance::power::PowerStateAnalyzer;
use crate::performance::window::ActiveWindowAnalyzer;
use std::sync::Mutex;

pub struct PerformanceManager {
    config: PerformanceConfig,
    power_analyzer: Box<dyn PowerStateAnalyzer>,
    window_analyzer: Box<dyn ActiveWindowAnalyzer>,
    
    consecutive_static_screens: Mutex<u32>,
}

impl PerformanceManager {
    pub fn new(
        config: PerformanceConfig,
        power_analyzer: Box<dyn PowerStateAnalyzer>,
        window_analyzer: Box<dyn ActiveWindowAnalyzer>,
    ) -> Self {
        Self {
            config,
            power_analyzer,
            window_analyzer,
            consecutive_static_screens: Mutex::new(0),
        }
    }

    /// Determines the optimal execution parameters for the current cycle.
    pub fn evaluate_performance_state(&self) -> PerformanceState {
        let power_state = self.power_analyzer.current_power_state();
        let is_fullscreen = self.window_analyzer.is_fullscreen_active();
        
        let mut policy = match power_state {
            PowerState::AC => self.config.ac_policy.clone(),
            PowerState::BatteryHigh => self.config.battery_high_policy.clone(),
            PowerState::BatteryLow => self.config.battery_low_policy.clone(),
            PowerState::BatterySaver => self.config.battery_saver_policy.clone(),
        };

        // Full-screen Option B policy: pause screen analysis entirely,
        // but let ambient keep running (at whatever rate the power state dictated,
        // or a default slow rate).
        if is_fullscreen {
            policy.pause_screen_analysis = true;
            // Optionally, slow down ambient further during full screen.
            // For now, we trust the power state's ambient rate.
        }

        // Apply static screen backoff to interval
        if !policy.pause_screen_analysis {
            let static_count = *self.consecutive_static_screens.lock().unwrap();
            if static_count > 0 {
                // Exponential backoff logic based on how long screen is static
                let backoff_multiplier = 1.0 + (static_count as f32 * 0.1);
                let new_interval = (policy.screen_analysis_interval_ms as f32 * backoff_multiplier) as u64;
                policy.screen_analysis_interval_ms = new_interval.min(self.config.static_screen_backoff_max_ms);
            }
        }

        PerformanceState {
            power_state,
            is_fullscreen_app_active: is_fullscreen,
            active_policy: policy,
        }
    }
    
    pub fn report_screen_changed(&self, changed: bool) {
        let mut static_screens = self.consecutive_static_screens.lock().unwrap();
        if changed {
            *static_screens = 0;
        } else {
            *static_screens = static_screens.saturating_add(1);
        }
    }
    
    pub fn get_diagnostics(&self) -> PerformanceDiagnostics {
        let state = self.evaluate_performance_state();
        let static_count = *self.consecutive_static_screens.lock().unwrap();
        let static_multiplier = 1.0 + (static_count as f32 * 0.1);
        
        PerformanceDiagnostics {
            current_power_state: state.power_state,
            is_fullscreen_active: state.is_fullscreen_app_active,
            screen_analysis_suspended: state.active_policy.pause_screen_analysis,
            current_screen_interval_ms: state.active_policy.screen_analysis_interval_ms,
            current_ambient_interval_ms: state.active_policy.ambient_interval_ms,
            static_screen_multiplier: static_multiplier,
            estimated_cpu_usage_pct: if state.active_policy.pause_screen_analysis { 0.05 } else { 0.2 },
        }
    }
}
