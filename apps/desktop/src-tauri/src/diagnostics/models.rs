use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Starting,
    Stopping,
    Warning,
    Degraded,
    Recovering,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemHealth {
    pub name: String,
    pub status: HealthStatus,
    pub last_update: u64,
    pub error_count: u32,
    pub warning_count: u32,
    pub average_duration_ms: u32,
    pub last_failure: Option<String>,
    pub recovery_attempts: u32,
    pub current_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsSnapshot {
    pub timestamp: u64,
    pub cpu_usage_percent: f32,
    pub ram_usage_mb: u32,
    pub thread_count: u32,
    pub queue_depth: u32,
    pub plugin_count: u32,
    pub subsystems: Vec<SubsystemHealth>,
}
