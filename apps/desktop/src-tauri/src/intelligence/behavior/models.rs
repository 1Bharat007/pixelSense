use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBehaviorSnapshot {
    pub active_application: String,
    pub session_duration_minutes: u32,
    pub manual_overrides_today: u32,
    pub ignored_recommendations: u32,
    pub profile_switches_today: u32,
    pub fullscreen_usage_minutes: u32,
    pub power_mode: String,
    pub monitor_usage_count: usize,
    pub average_transition_speed: String,
    pub preferred_brightness_range: (u8, u8),
}
