use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub onboarding: OnboardingSection,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionSection {
    pub enabled: bool,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrightnessSection {
    pub manual_override_timeout_ms: u64,
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
            },
            transition: TransitionSection {
                enabled: true,
                duration_ms: 500,
            },
            brightness: BrightnessSection {
                manual_override_timeout_ms: 3600000,
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

pub struct ConfigService {
    config_path: PathBuf,
    current_config: Arc<Mutex<AppConfig>>,
}

impl ConfigService {
    pub fn new(config_path: PathBuf) -> Self {
        let config = if config_path.exists() {
            let data = fs::read_to_string(&config_path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            AppConfig::default()
        };

        // Ensure file exists
        if !config_path.exists() {
            if let Ok(json) = serde_json::to_string_pretty(&config) {
                let _ = fs::write(&config_path, json);
            }
        }

        Self {
            config_path,
            current_config: Arc::new(Mutex::new(config)),
        }
    }

    pub fn get_config(&self) -> AppConfig {
        self.current_config.lock().unwrap().clone()
    }

    pub fn save_config(&self, new_config: AppConfig) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&new_config).map_err(|e| e.to_string())?;
        fs::write(&self.config_path, json).map_err(|e| e.to_string())?;
        
        *self.current_config.lock().unwrap() = new_config;
        
        // In the future: notify managers (AdaptiveBrightnessService, etc.)
        // For this sprint, configuration maps dynamically on next startup.

        Ok(())
    }
}

