use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub adaptive: AdaptiveSection,
    pub transition: TransitionSection,
    pub brightness: BrightnessSection,
    pub appearance: AppearanceSection,
    pub performance: PerformanceSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveSection {
    pub enabled: bool,
    pub confidence_threshold: f32,
    pub poll_interval_ms: Option<u64>,
    pub transition_interval_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionSection {
    pub enabled: bool,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrightnessSection {
    pub manual_override_timeout_ms: u64,
    pub reference_brightness: Option<u8>,
    pub reference_ambient_lux: Option<f32>,
    pub display_model: Option<String>,
    pub monitor_type: Option<String>,
    pub power_mode: Option<String>,
    pub timestamp: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSection {
    pub theme: String, // "System", "Light", "Dark"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSection {
    pub mode: String, // "PowerSaving", "Balanced", "Performance"
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            adaptive: AdaptiveSection {
                enabled: true,
                confidence_threshold: 0.5,
                poll_interval_ms: Some(1000),
                transition_interval_ms: Some(50),
            },
            transition: TransitionSection {
                enabled: true,
                duration_ms: 500,
            },
            brightness: BrightnessSection {
                manual_override_timeout_ms: 3600000,
                reference_brightness: None,
                reference_ambient_lux: None,
                display_model: None,
                monitor_type: None,
                power_mode: None,
                timestamp: None,
                profile: None,
            },
            appearance: AppearanceSection {
                theme: "System".into(),
            },
            performance: PerformanceSection {
                mode: "Balanced".into(),
            },
        }
    }
}
