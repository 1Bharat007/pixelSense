#[derive(Debug, Clone)]
pub struct DecisionConfig {
    pub minimum_brightness: u8,
    pub maximum_brightness: u8,
    pub maximum_step_change: u8,
    pub daytime_limit: u8,
    pub nighttime_limit: u8,
}

impl Default for DecisionConfig {
    fn default() -> Self {
        Self {
            minimum_brightness: 5,
            maximum_brightness: 100,
            maximum_step_change: 20,
            daytime_limit: 100,
            nighttime_limit: 40,
        }
    }
}
