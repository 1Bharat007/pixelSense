use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryPolicy {
    Retry,
    Restart,
    Fallback,
    Disable,
    Notify,
    Ignore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrashReason {
    Panic(String),
    Timeout,
    MemoryExhaustion,
    PluginError(String),
    PlatformError(String),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    pub subsystem_id: String,
    pub reason: CrashReason,
    pub timestamp: u64,
    pub correlation_id: Option<String>,
    pub policy_applied: RecoveryPolicy,
}
