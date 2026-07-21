use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Number of consecutive manual overrides before we consider it a "preference".
const LEARN_THRESHOLD: u32 = 3;

/// A learned brightness preference for a specific application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBrightnessRecord {
    /// The application name (process basename, e.g. "Code.exe").
    pub app_name: String,
    /// The learned preferred brightness percentage (0–100).
    pub preferred_brightness: u8,
    /// How many times the user has manually set brightness for this app.
    pub override_count: u32,
    /// Unix timestamp (ms) of last update.
    pub last_updated_ms: u64,
}

/// AppBrightnessMemory learns per-application brightness preferences from
/// repeated manual overrides. It never stores cloud data — all memory is local.
///
/// Responsibility: Learn what brightness level each application prefers,
/// based on the user's manual corrections, and expose that for the Decision Engine.
pub struct AppBrightnessMemory {
    /// Confirmed preferences (override_count >= LEARN_THRESHOLD).
    preferences: HashMap<String, AppBrightnessRecord>,
    /// In-progress tally: pending overrides not yet confirmed as preferences.
    pending: HashMap<String, (u8, u32)>, // (last brightness, count)
    /// Path to persist memory (optional).
    persist_path: Option<std::path::PathBuf>,
}

impl AppBrightnessMemory {
    pub fn new() -> Self {
        Self {
            preferences: HashMap::new(),
            pending: HashMap::new(),
            persist_path: None,
        }
    }

    /// Load from disk if a path is configured.
    pub fn load(path: std::path::PathBuf) -> Self {
        let preferences = if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str::<HashMap<String, AppBrightnessRecord>>(&s).ok())
                .unwrap_or_default()
        } else {
            HashMap::new()
        };

        Self {
            preferences,
            pending: HashMap::new(),
            persist_path: Some(path),
        }
    }

    /// Called when the user manually sets brightness while `app` is in the foreground.
    /// Accumulates overrides. After `LEARN_THRESHOLD` overrides for the same app,
    /// the preference is confirmed and saved.
    pub fn record_override(&mut self, app: &str, brightness: u8) {
        if app.is_empty() || app == "Unknown" {
            return;
        }

        let app_key = normalize_app_name(app);
        let entry = self.pending.entry(app_key.clone()).or_insert((brightness, 0));

        // If the new brightness is within 5% of last recorded, count it.
        if (brightness as i32 - entry.0 as i32).abs() <= 5 {
            entry.1 += 1;
        } else {
            // Reset with new target
            *entry = (brightness, 1);
        }

        if entry.1 >= LEARN_THRESHOLD {
            // Promote to confirmed preference
            let confirmed_brightness = entry.0;
            self.pending.remove(&app_key);
            let now_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;

            self.preferences.insert(app_key.clone(), AppBrightnessRecord {
                app_name: app_key,
                preferred_brightness: confirmed_brightness,
                override_count: LEARN_THRESHOLD,
                last_updated_ms: now_ms,
            });

            self.save();
        }
    }

    /// Returns the learned preferred brightness for `app`, if any.
    pub fn get_preference(&self, app: &str) -> Option<u8> {
        let key = normalize_app_name(app);
        self.preferences.get(&key).map(|r| r.preferred_brightness)
    }

    /// Returns all confirmed preferences (for UI display).
    pub fn all_preferences(&self) -> Vec<&AppBrightnessRecord> {
        self.preferences.values().collect()
    }

    fn save(&self) {
        if let Some(path) = &self.persist_path {
            if let Ok(json) = serde_json::to_string_pretty(&self.preferences) {
                let _ = std::fs::create_dir_all(path.parent().unwrap_or(std::path::Path::new(".")));
                let _ = std::fs::write(path, json);
            }
        }
    }
}

impl Default for AppBrightnessMemory {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_app_name(app: &str) -> String {
    app.to_lowercase()
        .trim_end_matches(".exe")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_preference_initially() {
        let memory = AppBrightnessMemory::new();
        assert!(memory.get_preference("code").is_none());
    }

    #[test]
    fn test_preference_learned_after_threshold() {
        let mut memory = AppBrightnessMemory::new();
        for _ in 0..LEARN_THRESHOLD {
            memory.record_override("Code.exe", 40);
        }
        assert_eq!(memory.get_preference("Code.exe"), Some(40));
    }

    #[test]
    fn test_reset_on_large_change() {
        let mut memory = AppBrightnessMemory::new();
        memory.record_override("chrome", 50);
        memory.record_override("chrome", 80); // Big jump — resets count
        // Only 1 count toward 80, shouldn't be confirmed yet
        assert!(memory.get_preference("chrome").is_none());
    }

    #[test]
    fn test_normalize_exe_extension() {
        let mut memory = AppBrightnessMemory::new();
        for _ in 0..LEARN_THRESHOLD {
            memory.record_override("Code.exe", 45);
        }
        // Should be accessible with or without .exe
        assert_eq!(memory.get_preference("code.exe"), Some(45));
        assert_eq!(memory.get_preference("code"), Some(45));
    }
}
