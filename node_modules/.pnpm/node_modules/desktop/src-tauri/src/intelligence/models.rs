use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistorySummary {
    pub total_events: usize,
    pub brightness_changes_today: u32,
    pub manual_overrides_today: u32,
    pub longest_session_minutes: u32,
    pub average_ambient_lux: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceContext {
    pub current_time_ms: u64,
    pub comfort_profile: String,
    pub history_summary: HistorySummary,
    pub current_ambient_lux: f32,
    pub current_screen_luminance: f32,
    pub worker_running: bool,
    pub performance_policy: String,
    pub active_application: String,
    pub active_display_id: String,
    pub confidence_score: f32,
}
