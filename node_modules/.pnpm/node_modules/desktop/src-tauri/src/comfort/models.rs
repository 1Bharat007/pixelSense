use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComfortProfile {
    pub profile_id: String,
    pub profile_name: String,
    pub display_identifier: String,
    
    // Environmental conditions
    pub ambient_light: f32,
    pub average_screen_luminance: f32,
    
    // User preference
    pub monitor_brightness: u8,
    
    // Metadata
    pub comfort_timestamp: u64,
    pub calibration_quality: f32,
    pub schema_version: u32,
    pub algorithm_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchResult {
    pub matched_profile: ComfortProfile,
    pub similarity_score: f32,
    pub distance: f32,
    pub reason: String,
}
