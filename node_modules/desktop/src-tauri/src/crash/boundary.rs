use std::panic;
use crate::crash::models::{RecoveryPolicy, CrashReport, CrashReason};
use crate::background::models::now_ms;

pub struct CrashBoundary {
    subsystem_id: String,
    policy: RecoveryPolicy,
}

impl CrashBoundary {
    pub fn new(subsystem_id: &str, policy: RecoveryPolicy) -> Self {
        Self {
            subsystem_id: subsystem_id.to_string(),
            policy,
        }
    }

    /// Executes a closure catching unwinds (panics) securely.
    pub fn execute<F, R>(&self, f: F) -> Result<R, CrashReport>
    where
        F: FnOnce() -> R + panic::UnwindSafe,
    {
        let result = panic::catch_unwind(f);
        
        match result {
            Ok(val) => Ok(val),
            Err(err) => {
                let msg = if let Some(s) = err.downcast_ref::<&str>() {
                    (*s).to_string()
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic".to_string()
                };

                let report = CrashReport {
                    subsystem_id: self.subsystem_id.clone(),
                    reason: CrashReason::Panic(msg),
                    timestamp: now_ms(),
                    correlation_id: None, // Can be injected via thread-local if needed
                    policy_applied: self.policy.clone(),
                };

                Err(report)
            }
        }
    }
}
