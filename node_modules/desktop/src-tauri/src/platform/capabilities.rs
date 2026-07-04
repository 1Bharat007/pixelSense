use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlatformCapabilities {
    pub os_version: String,
    pub ambient_sensor: bool,
    pub desktop_duplication: bool,
    pub ddc_ci: bool,
    pub internal_monitor_brightness: bool,
    pub hdr: bool,
    pub night_light_detection: bool,
    pub refresh_rate_query: bool,
    pub power_state: bool,
    pub window_tracking: bool,
    pub display_enumeration: bool,
    
    // Expanded Capabilities
    pub plugin_support: bool,
    pub updater: bool,
    pub installer: bool,
    pub notifications: bool,
    pub history: bool,
    pub ai_ready: bool,
}

impl PlatformCapabilities {
    pub fn detect() -> Self {
        // Mock detection
        Self::default()
    }
}

impl Default for PlatformCapabilities {
    fn default() -> Self {
        Self {
            os_version: "10.0.19041".into(),
            ambient_sensor: false,
            desktop_duplication: false,
            ddc_ci: false,
            internal_monitor_brightness: false,
            hdr: false,
            night_light_detection: false,
            refresh_rate_query: false,
            power_state: false,
            window_tracking: false,
            display_enumeration: false,
            plugin_support: true,
            updater: true,
            installer: true,
            notifications: true,
            history: true,
            ai_ready: false,
        }
    }
}
