use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeAnalytics {
    pub current_comfort_score: u8,
    pub active_monitors: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyAnalytics {
    pub average_comfort_score: u8,
    pub total_transitions: u32,
    pub manual_overrides: u32,
    pub average_ambient_lux: f32,
    pub average_screen_luminance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyAnalytics {
    pub average_comfort_score: u8,
    pub active_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyAnalytics {
    pub average_comfort_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSnapshot {
    pub realtime: RealtimeAnalytics,
    pub daily: DailyAnalytics,
    pub weekly: WeeklyAnalytics,
    pub monthly: MonthlyAnalytics,
}
