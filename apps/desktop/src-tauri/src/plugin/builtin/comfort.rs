use crate::plugin::traits::{PixelSensePlugin, RecommendationPlugin};
use crate::plugin::models::{PluginManifest, PluginHealth};
use crate::plugin::context::PluginContext;

pub struct ComfortBuiltinPlugin {
    manifest: PluginManifest,
    health: PluginHealth,
}

impl ComfortBuiltinPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "com.pixelsense.builtin.comfort".into(),
                name: "Comfort Engine".into(),
                description: "Built-in visual comfort orchestration".into(),
                author: "PixelSense".into(),
                version: "1.0.0".into(),
                min_sdk_version: "1.0.0".into(),
                capabilities_requested: vec!["ReadComfort".into(), "ProvideRecommendation".into()],
                dependencies: vec![],
            },
            health: PluginHealth::Healthy,
        }
    }
}

impl PixelSensePlugin for ComfortBuiltinPlugin {
    fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    fn health(&self) -> PluginHealth {
        self.health.clone()
    }

    fn initialize(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn on_tick(&mut self, _context: &PluginContext) -> Result<(), String> {
        Ok(())
    }
}

impl RecommendationPlugin for ComfortBuiltinPlugin {
    fn get_brightness_recommendation(&self, context: &PluginContext) -> Option<u8> {
        // Here we would call the existing VisualComfortEngine passing the summarized data.
        // For demonstration of the SDK architecture, we return a mocked recommendation based on ambient lux.
        if context.ambient_summary.current_lux < 50.0 {
            Some(20)
        } else {
            Some(80)
        }
    }
}
