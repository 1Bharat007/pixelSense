#[derive(Debug, Clone)]
pub struct AmbientConfig {
    pub minimum_poll_interval: u64, // ms
    pub preferred_poll_interval: u64, // ms
    pub maximum_poll_interval: u64, // ms
    pub minimum_change_threshold: f32,
    pub smoothing_enabled: bool,
    pub fallback_enabled: bool,
}

impl Default for AmbientConfig {
    fn default() -> Self {
        Self {
            minimum_poll_interval: 100,
            preferred_poll_interval: 1000,
            maximum_poll_interval: 5000,
            minimum_change_threshold: 5.0,
            smoothing_enabled: true,
            fallback_enabled: true,
        }
    }
}
