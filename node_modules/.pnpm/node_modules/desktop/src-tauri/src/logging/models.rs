use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Information,
    Warning,
    Error,
    Critical,
    Silent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogCategory {
    Platform,
    Brightness,
    Ambient,
    ScreenAnalysis,
    BackgroundWorker,
    History,
    Experience,
    Intelligence,
    Dashboard,
    Plugin,
    Performance,
    Recovery,
    Updater,
    Installer,
    Configuration,
    Diagnostics,
    Developer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub session_id: String,
    pub correlation_id: Option<String>,
    pub worker_id: Option<String>,
    pub display_id: Option<String>,
    pub thread_id: String,
    pub plugin_id: Option<String>,
    pub category: LogCategory,
    pub level: LogLevel,
    pub duration_ms: Option<u32>,
    pub message: String,
    pub context: String, // JSON serialization of context
}
