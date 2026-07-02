use crate::comfort::error::ComfortError;
use crate::comfort::models::{ComfortProfile, MatchResult};
use crate::comfort::strategies::MatchingStrategy;

pub struct NearestNeighborStrategy;

impl NearestNeighborStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NearestNeighborStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchingStrategy for NearestNeighborStrategy {
    fn find_best_match(
        &self,
        ambient_light: f32,
        average_screen_luminance: f32,
        profiles: &[ComfortProfile],
    ) -> Result<MatchResult, ComfortError> {
        if profiles.is_empty() {
            return Err(ComfortError::ProfileNotFound("No profiles available to match against".into()));
        }

        let mut best_match: Option<&ComfortProfile> = None;
        let mut min_distance = f32::MAX;

        for profile in profiles {
            // Euclidean distance in 2D space (lux, luminance). 
            // In a real system, these would be normalized first (e.g., lux log scaled 0-1, luminance 0-1).
            // For now, we perform raw euclidean distance.
            let d_ambient = profile.ambient_light - ambient_light;
            let d_luminance = profile.average_screen_luminance - average_screen_luminance;
            
            let distance = (d_ambient * d_ambient + d_luminance * d_luminance).sqrt();

            if distance < min_distance {
                min_distance = distance;
                best_match = Some(profile);
            }
        }

        let matched = best_match.unwrap().clone();

        // Calculate a pseudo similarity score (1.0 = exact match, decaying as distance increases)
        let similarity_score = (1.0 - (min_distance / 100.0)).clamp(0.0, 1.0);

        Ok(MatchResult {
            matched_profile: matched,
            similarity_score,
            distance: min_distance,
            reason: format!("Matched via Nearest Neighbor with distance {:.2}", min_distance),
        })
    }
}
