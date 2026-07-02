use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::plugin::traits::PixelSensePlugin;
use crate::plugin::models::{PluginManifest, PluginState, PluginHealth};

pub struct PluginRegistry {
    plugins: RwLock<HashMap<String, Arc<RwLock<dyn PixelSensePlugin>>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, plugin: Arc<RwLock<dyn PixelSensePlugin>>) -> Result<(), String> {
        let manifest = plugin.read().unwrap().manifest().clone();
        let mut write_lock = self.plugins.write().unwrap();
        
        if write_lock.contains_key(&manifest.id) {
            return Err(format!("Plugin {} already registered", manifest.id));
        }

        write_lock.insert(manifest.id.clone(), plugin);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<Arc<RwLock<dyn PixelSensePlugin>>> {
        let read_lock = self.plugins.read().unwrap();
        read_lock.get(id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<RwLock<dyn PixelSensePlugin>>> {
        let read_lock = self.plugins.read().unwrap();
        read_lock.values().cloned().collect()
    }
}

pub struct PluginManager {
    registry: Arc<PluginRegistry>,
}

impl PluginManager {
    pub fn new(registry: Arc<PluginRegistry>) -> Self {
        Self { registry }
    }

    pub fn initialize_all(&self) -> Result<(), String> {
        for plugin_lock in self.registry.all() {
            let mut plugin = plugin_lock.write().unwrap();
            let id = plugin.manifest().id.clone();
            
            if let Err(e) = plugin.initialize() {
                // Should emit event here, for now we log and continue
                println!("Failed to initialize plugin {}: {}", id, e);
            }
        }
        Ok(())
    }
}
