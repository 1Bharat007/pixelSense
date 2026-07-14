use std::collections::VecDeque;
use std::sync::RwLock;
use crate::crash::models::{CrashReport, RecoveryPolicy};

pub struct CrashManager {
    reports: RwLock<VecDeque<CrashReport>>,
}

impl CrashManager {
    pub fn new() -> Self {
        Self {
            reports: RwLock::new(VecDeque::new()),
        }
    }

    pub fn handle_crash(&self, report: CrashReport) {
        let mut reports = self.reports.write().unwrap();
        reports.push_back(report.clone());
        if reports.len() > 100 {
            reports.pop_front();
        }

        // Apply recovery policy based on report
        match report.policy_applied {
            RecoveryPolicy::Disable => {
                // E.g., if a plugin crashes, disable it via PluginManager
                println!("CrashManager: Disabling subsystem {}", report.subsystem_id);
            },
            RecoveryPolicy::Restart => {
                println!("CrashManager: Scheduling restart for {}", report.subsystem_id);
            },
            RecoveryPolicy::Fallback => {
                println!("CrashManager: Falling back for {}", report.subsystem_id);
            },
            _ => {
                println!("CrashManager: Handled {} with {:?}", report.subsystem_id, report.policy_applied);
            }
        }
    }
}
