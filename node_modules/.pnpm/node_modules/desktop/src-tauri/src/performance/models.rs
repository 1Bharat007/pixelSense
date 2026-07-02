#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PowerState {
    AC,
    BatteryHigh,
    BatteryLow,
    BatterySaver,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationPolicy {
    pub screen_analysis_interval_ms: u64,
    pub ambient_interval_ms: u64,
    pub pause_screen_analysis: bool,
    pub pause_ambient: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceState {
    pub power_state: PowerState,
    pub is_fullscreen_app_active: bool,
    pub active_policy: OptimizationPolicy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceDiagnostics {
    pub current_power_state: PowerState,
    pub is_fullscreen_active: bool,
    pub screen_analysis_suspended: bool,
    pub current_screen_interval_ms: u64,
    pub current_ambient_interval_ms: u64,
    pub static_screen_multiplier: f32,
    pub estimated_cpu_usage_pct: f32,
}
