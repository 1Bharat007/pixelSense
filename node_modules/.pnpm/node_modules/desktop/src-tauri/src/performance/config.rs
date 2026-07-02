use crate::performance::models::OptimizationPolicy;

#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub ac_policy: OptimizationPolicy,
    pub battery_high_policy: OptimizationPolicy,
    pub battery_low_policy: OptimizationPolicy,
    pub battery_saver_policy: OptimizationPolicy,
    
    // Backoff settings for static screen
    pub static_screen_backoff_base_ms: u64,
    pub static_screen_backoff_max_ms: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            ac_policy: OptimizationPolicy {
                screen_analysis_interval_ms: 500,
                ambient_interval_ms: 500,
                pause_screen_analysis: false,
                pause_ambient: false,
            },
            battery_high_policy: OptimizationPolicy {
                screen_analysis_interval_ms: 1000,
                ambient_interval_ms: 1000,
                pause_screen_analysis: false,
                pause_ambient: false,
            },
            battery_low_policy: OptimizationPolicy {
                screen_analysis_interval_ms: 2500,
                ambient_interval_ms: 1500,
                pause_screen_analysis: false,
                pause_ambient: false,
            },
            battery_saver_policy: OptimizationPolicy {
                screen_analysis_interval_ms: 0, // Paused
                ambient_interval_ms: 3000,
                pause_screen_analysis: true,
                pause_ambient: false, // Ambient still runs slowly
            },
            static_screen_backoff_base_ms: 1000,
            static_screen_backoff_max_ms: 10000,
        }
    }
}
