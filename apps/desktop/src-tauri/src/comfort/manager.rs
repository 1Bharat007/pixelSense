use crate::comfort::error::ComfortError;
use crate::comfort::models::{ComfortProfile, MatchResult};
use crate::comfort::storage::ComfortStorage;
use crate::comfort::strategies::MatchingStrategy;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ComfortManager {
    storage: Box<dyn ComfortStorage>,
    matching_strategy: Box<dyn MatchingStrategy>,
}

impl ComfortManager {
    pub fn new(
        storage: Box<dyn ComfortStorage>,
        matching_strategy: Box<dyn MatchingStrategy>,
    ) -> Self {
        Self {
            storage,
            matching_strategy,
        }
    }

    pub fn lock_comfort(
        &self,
        display_id: String,
        ambient_light: f32,
        average_screen_luminance: f32,
        monitor_brightness: u8,
        profile_name: Option<String>,
    ) -> Result<ComfortProfile, ComfortError> {
        // We use a basic timestamp as UUID fallback since we don't have uuid crate imported natively.
        // In production, use Uuid::new_v4().to_string()
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let profile_id = format!("prof-{}", timestamp);

        let profile = ComfortProfile {
            profile_id,
            profile_name: profile_name.unwrap_or_else(|| "Locked Comfort".into()),
            display_identifier: display_id,
            ambient_light,
            average_screen_luminance,
            monitor_brightness,
            comfort_timestamp: timestamp as u64,
            calibration_quality: 1.0, // Manual lock implies highest confidence
            schema_version: 1,
            algorithm_version: 1,
        };

        self.storage.save_profile(&profile)?;
        Ok(profile)
    }

    pub fn recommend_comfort(
        &self,
        display_id: &str,
        ambient_light: f32,
        average_screen_luminance: f32,
    ) -> Result<MatchResult, ComfortError> {
        let profiles = self.storage.load_profiles_for_display(display_id)?;
        
        self.matching_strategy.find_best_match(
            ambient_light,
            average_screen_luminance,
            &profiles,
        )
    }
}
