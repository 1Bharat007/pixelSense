use crate::intelligence::models::IntelligenceContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComfortScoreResult {
    pub total_score: u8,
    pub ambient_match: u8,
    pub screen_luminance: u8,
    pub brightness_level: u8,
    pub contrast: u8,
    pub blue_light: u8,
    pub time_of_day: u8,
    pub manual_preference: u8,
}

pub struct ComfortScoreEngine;

impl ComfortScoreEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate(&self, context: &IntelligenceContext) -> ComfortScoreResult {
        // Weighted calculation:
        // Ambient Match 30%
        // Screen Luminance 20%
        // Brightness 15%
        // Contrast 10%
        // Blue Light 10%
        // Time of Day 5%
        // Manual Preference 10%

        // 1. Ambient Match (30%) - Assuming confidence translates roughly to match
        let ambient_match = (context.confidence_score * 30.0) as u8;

        // 2. Screen Luminance (20%) - Normalize luminance 0-100 to a score
        let lum_score = if context.current_screen_luminance > 20.0 && context.current_screen_luminance < 80.0 {
            20
        } else {
            10
        };

        // 3. Brightness Level (15%) - Just a placeholder since target is dynamic
        let brightness_level = 15;

        // 4. Contrast (10%)
        let contrast = 10;

        // 5. Blue Light (10%)
        let blue_light = 10;

        // 6. Time of Day (5%)
        let time_of_day = 5;

        // 7. Manual Preference (10%)
        let overrides = context.history_summary.manual_overrides_today;
        let manual_preference = if overrides > 5 { 5 } else { 10 - overrides as u8 };

        let total = ambient_match
            + lum_score
            + brightness_level
            + contrast
            + blue_light
            + time_of_day
            + manual_preference;

        ComfortScoreResult {
            total_score: total.min(100),
            ambient_match,
            screen_luminance: lum_score,
            brightness_level,
            contrast,
            blue_light,
            time_of_day,
            manual_preference,
        }
    }
}
