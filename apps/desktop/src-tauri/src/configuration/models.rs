use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub system: SystemSection,
    pub adaptive: AdaptiveSection,
    pub transition: TransitionSection,
    pub brightness: BrightnessSection,
    pub appearance: AppearanceSection,
    pub performance: PerformanceSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSection {
    #[serde(default = "default_true")]
    pub start_with_windows: bool,
    #[serde(default = "default_true")]
    pub run_in_background: bool,
}

fn default_true() -> bool { true }

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
    pub hysteresis_pct: u8,
    pub easing_curve: String,
    /// How long to wait after a transition completes before starting another.
    /// This eliminates oscillation between two nearby brightness levels.
    #[serde(default = "default_cooldown_ms")]
    pub cooldown_ms: u64,
}

fn default_cooldown_ms() -> u64 { 200 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComfortProfile {
    pub reference_brightness: u8,
    pub reference_lux: f32,
    pub min_brightness: u8,
    pub max_brightness: u8,
    pub adaptation_speed: String,
    pub transition_curve: String,
    pub sensitivity: f32,
    pub manual_override_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrightnessSection {
    pub manual_override_suspend_ms: u64,
    pub comfort_profile: Option<ComfortProfile>,
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
            system: SystemSection {
                start_with_windows: true,
                run_in_background: true,
            },
            adaptive: AdaptiveSection {
                enabled: false,
                confidence_threshold: 0.5,
                poll_interval_ms: Some(1000),
                transition_interval_ms: Some(50),
            },
            transition: TransitionSection {
                enabled: true,
                duration_ms: 250,       // 250ms base (perceptually smooth, fast response)
                hysteresis_pct: 3,      // 3% minimum change threshold
                easing_curve: "Natural".into(), // EaseOutCubic — matches human perception
                cooldown_ms: 200,       // 200ms cooldown prevents hardware spam
            },
            brightness: BrightnessSection {
                manual_override_suspend_ms: 30000,
                comfort_profile: None,
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
