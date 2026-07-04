use crate::plugin::models::{PluginManifest, PluginHealth};
use crate::plugin::context::PluginContext;

/// The base trait all plugins must implement.
pub trait PixelSensePlugin: Send + Sync {
    fn manifest(&self) -> &PluginManifest;
    fn health(&self) -> PluginHealth;
    
    fn initialize(&mut self) -> Result<(), String>;
    fn shutdown(&mut self) -> Result<(), String>;
    
    /// Called periodically with the read-only context.
    fn on_tick(&mut self, context: &PluginContext) -> Result<(), String>;
}

/// A specialized trait for plugins that provide brightness recommendations.
pub trait RecommendationPlugin: PixelSensePlugin {
    fn get_brightness_recommendation(&self, context: &PluginContext) -> Option<u8>;
}

/// A specialized trait for plugins that generate dashboard widgets.
pub trait DashboardWidgetPlugin: PixelSensePlugin {
    fn get_widget_json(&self) -> String;
}
