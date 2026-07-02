use crate::visual_comfort::models::{
    ComfortConfig, ComfortRecommendation, RecommendationAction, VisualComfortContext, VisualComfortResult,
};
use crate::visual_comfort::strategies::CompensationStrategy;

pub struct BasicCompensationStrategy;

impl BasicCompensationStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

impl CompensationStrategy for BasicCompensationStrategy {
    fn calculate_compensation(
        &self,
        context: &VisualComfortContext,
        config: &ComfortConfig,
    ) -> VisualComfortResult {
        let profile = match &context.current_comfort_profile {
            Some(p) => p,
            None => {
                return VisualComfortResult {
                    recommendation: ComfortRecommendation {
                        recommended_brightness: context.current_monitor_brightness,
                        confidence: 0.0,
                        reason: "No comfort profile available".into(),
                        action: RecommendationAction::NoChange,
                    },
                    comfort_delta: 0.0,
                    predicted_eye_comfort: 50.0,
                    processing_time_ms: 1,
                }
            }
        };

        if let Some(luminance) = context.screen_luminance {
            // Simplified logic: If the screen is twice as bright mathematically as when
            // the user locked the profile, the screen brightness should be reduced.
            // locked_emitted = profile.average_screen_luminance * (profile.monitor_brightness / 100)
            // current_emitted = current_luminance * (current_brightness / 100)

            let locked_emitted = profile.average_screen_luminance * (profile.monitor_brightness as f32 / 100.0);
            
            // To maintain locked_emitted, new_brightness = (locked_emitted / current_luminance) * 100
            let mut target_brightness_f = if luminance > 0.0 {
                (locked_emitted / luminance) * 100.0
            } else {
                profile.monitor_brightness as f32
            };

            target_brightness_f = target_brightness_f.clamp(config.minimum_brightness as f32, config.maximum_brightness as f32);
            
            // Apply maximum step change limit (though this is technically also transition logic, 
            // the calculation engine provides the clamped *recommendation*)
            let mut recommended_brightness = target_brightness_f as u8;
            
            let diff = (context.current_monitor_brightness as i16 - recommended_brightness as i16).abs() as u8;
            
            let action = if diff < config.minimum_change_threshold {
                recommended_brightness = context.current_monitor_brightness;
                RecommendationAction::NoChange
            } else if context.transition_enabled {
                RecommendationAction::SmoothTransition
            } else {
                RecommendationAction::ImmediateTransition
            };

            VisualComfortResult {
                recommendation: ComfortRecommendation {
                    recommended_brightness,
                    confidence: context.confidence,
                    reason: "Calculated inverse proportional brightness based on luminance shift".into(),
                    action,
                },
                comfort_delta: diff as f32,
                predicted_eye_comfort: 90.0,
                processing_time_ms: 2,
            }
        } else {
            VisualComfortResult {
                recommendation: ComfortRecommendation {
                    recommended_brightness: context.current_monitor_brightness,
                    confidence: 0.0,
                    reason: "Luminance data unavailable".into(),
                    action: RecommendationAction::NoChange,
                },
                comfort_delta: 0.0,
                predicted_eye_comfort: 50.0,
                processing_time_ms: 1,
            }
        }
    }
}
