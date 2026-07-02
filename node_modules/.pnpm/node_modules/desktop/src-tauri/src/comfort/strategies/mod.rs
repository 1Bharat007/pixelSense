pub mod nearest_neighbor;

use crate::comfort::error::ComfortError;
use crate::comfort::models::{ComfortProfile, MatchResult};

pub trait MatchingStrategy: Send + Sync {
    fn find_best_match(
        &self,
        ambient_light: f32,
        average_screen_luminance: f32,
        profiles: &[ComfortProfile],
    ) -> Result<MatchResult, ComfortError>;
}
