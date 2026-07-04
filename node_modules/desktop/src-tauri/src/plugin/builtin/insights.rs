use crate::plugin::traits::{PixelSensePlugin, DashboardWidgetPlugin};
use crate::plugin::models::{PluginManifest, PluginHealth};
use crate::plugin::context::PluginContext;

pub struct InsightsBuiltinPlugin {
    manifest: PluginManifest,
    health: PluginHealth,
}

impl InsightsBuiltinPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "com.pixelsense.builtin.insights".into(),
                name: "Insights Engine".into(),
                description: "Built-in analytics and dashboard insights".into(),
                author: "PixelSense".into(),
                version: "1.0.0".into(),
                min_sdk_version: "1.0.0".into(),
                capabilities_requested: vec!["ReadHistory".into(), "ProvideWidget".into()],
                dependencies: vec![],
            },
            health: PluginHealth::Healthy,
        }
    }
}

impl PixelSensePlugin for InsightsBuiltinPlugin {
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

impl DashboardWidgetPlugin for InsightsBuiltinPlugin {
    fn get_widget_json(&self) -> String {
        r#"{
            "id": "insights_summary",
            "title": "Daily Insights",
            "type": "chart",
            "data": [10, 20, 15, 30]
        }"#.into()
    }
}
