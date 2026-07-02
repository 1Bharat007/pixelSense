use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub enum FeatureFlagType {
    Runtime,
    BetaOnly,
    DeveloperOnly,
    Experimental,
    PluginControlled(String), // Plugin ID
}

#[derive(Debug, Clone)]
pub struct FeatureFlag {
    pub key: String,
    pub flag_type: FeatureFlagType,
    pub enabled_by_default: bool,
}

pub struct FeatureFlagRegistry {
    flags: RwLock<HashMap<String, FeatureFlag>>,
    overrides: RwLock<HashMap<String, bool>>,
}

impl FeatureFlagRegistry {
    pub fn new() -> Self {
        Self {
            flags: RwLock::new(HashMap::new()),
            overrides: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, flag: FeatureFlag) {
        let mut flags = self.flags.write().unwrap();
        flags.insert(flag.key.clone(), flag);
    }

    pub fn set_override(&self, key: &str, enabled: bool) {
        let mut overrides = self.overrides.write().unwrap();
        overrides.insert(key.to_string(), enabled);
    }

    pub fn is_enabled(&self, key: &str) -> bool {
        if let Some(&overridden) = self.overrides.read().unwrap().get(key) {
            return overridden;
        }
        
        self.flags.read().unwrap().get(key).map(|f| f.enabled_by_default).unwrap_or(false)
    }
}
