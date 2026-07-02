use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub min_sdk_version: String,
    pub capabilities_requested: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginHealth {
    Healthy,
    Warning(String),
    Degraded(String),
    Disabled,
    Failed(String),
    Recovering,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginState {
    Discovered,
    Validated,
    PermissionChecked,
    Initialized,
    Running,
    Paused,
    Disabled,
    Recovering,
    Stopped,
    Unloaded,
}

#[derive(Debug, Clone)]
pub struct PluginEvent {
    pub plugin_id: String,
    pub event_type: PluginEventType,
}

#[derive(Debug, Clone)]
pub enum PluginEventType {
    StateChanged(PluginState),
    HealthChanged(PluginHealth),
    Custom(String),
}
