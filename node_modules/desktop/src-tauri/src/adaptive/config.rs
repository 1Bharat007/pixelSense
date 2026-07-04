#[derive(Debug, Clone)]
pub struct AdaptiveConfig {
    pub adaptive_enabled: bool,
    pub transition_enabled: bool,
    pub confidence_threshold: f32,
    pub minimum_update_interval_ms: u64,
    pub manual_override_timeout_ms: u64,
    pub transition_duration_ms: u64,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            adaptive_enabled: true,
            transition_enabled: true,
            confidence_threshold: 0.5,
            minimum_update_interval_ms: 1000,
            manual_override_timeout_ms: 3600000,
            transition_duration_ms: 500,
        }
    }
}
