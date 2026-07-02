use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::diagnostics::models::{SubsystemHealth, DiagnosticsSnapshot};

pub trait DiagnosticsProvider: Send + Sync {
    fn get_health(&self) -> SubsystemHealth;
}

pub struct DiagnosticsRegistry {
    providers: RwLock<HashMap<String, Arc<dyn DiagnosticsProvider>>>,
}

impl DiagnosticsRegistry {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, id: &str, provider: Arc<dyn DiagnosticsProvider>) {
        let mut providers = self.providers.write().unwrap();
        providers.insert(id.to_string(), provider);
    }

    pub fn generate_snapshot(&self) -> DiagnosticsSnapshot {
        let providers = self.providers.read().unwrap();
        let mut subsystems = Vec::new();
        
        for provider in providers.values() {
            subsystems.push(provider.get_health());
        }

        DiagnosticsSnapshot {
            timestamp: crate::background::models::now_ms(),
            cpu_usage_percent: 0.0, // Mocked for architecture scaffold
            ram_usage_mb: 0,
            thread_count: 0,
            queue_depth: 0,
            plugin_count: 0,
            subsystems,
        }
    }
}
