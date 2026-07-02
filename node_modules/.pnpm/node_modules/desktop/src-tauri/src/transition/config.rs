/// Configuration for transition timing.
#[derive(Debug, Clone)]
pub struct TransitionConfig {
    /// The interval in milliseconds between brightness updates.
    /// Default is 16ms (approx 60fps).
    pub tick_interval_ms: u64,
}

impl Default for TransitionConfig {
    fn default() -> Self {
        Self {
            tick_interval_ms: 16,
        }
    }
}
