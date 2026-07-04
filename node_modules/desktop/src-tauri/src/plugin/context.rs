use serde::{Serialize, Deserialize};

/// Summarized read-only context passed to plugins to prevent direct backend manager access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    pub display_summary: DisplaySummary,
    pub ambient_summary: AmbientSummary,
    pub comfort_summary: ComfortSummary,
    pub performance_summary: PerformanceSummary,
    pub configuration_snapshot: String, // JSON serialization of relevant configs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySummary {
    pub display_count: usize,
    pub active_display_id: String,
    pub current_brightness: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientSummary {
    pub current_lux: f32,
    pub environment: String, // e.g. "Office", "DarkRoom"
    pub is_stable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComfortSummary {
    pub current_score: u8,
    pub user_fatigue_estimate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub battery_saver_active: bool,
    pub cpu_budget_usage_percent: f32,
}
