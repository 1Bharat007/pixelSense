use crate::decision::error::DecisionError;
use crate::decision::models::{ComfortLevel, DecisionContext, DecisionResult, TimeOfDay};
use crate::decision::strategies::DecisionStrategy;

pub struct DefaultDecisionStrategy;

impl DefaultDecisionStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultDecisionStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl DecisionStrategy for DefaultDecisionStrategy {
    fn calculate_brightness(&self, context: &DecisionContext) -> Result<DecisionResult, DecisionError> {
        let mut confidence: f32 = 0.5;
        let mut reasoning = String::new();
        let mut target_brightness: f32;

        // 1. Process Ambient Light or Fallback to TimeOfDay
        if let Some(ambient) = &context.ambient_light {
            confidence += 0.3; // High confidence when we have sensor data
            
            if ambient.lux < 10.0 {
                target_brightness = 15.0; // Dark room
                reasoning.push_str("Dark room detected. ");
            } else if ambient.lux > 1000.0 {
                target_brightness = 90.0; // Bright room
                reasoning.push_str("Bright room detected. ");
            } else {
                target_brightness = 50.0; // Medium room
                reasoning.push_str("Medium light room detected. ");
            }
        } else {
            // Fallback to time of day
            target_brightness = match context.time_of_day {
                TimeOfDay::Morning => 60.0,
                TimeOfDay::Day => 80.0,
                TimeOfDay::Evening => 40.0,
                TimeOfDay::Night => 20.0,
            };
            reasoning.push_str("Fallback to time-of-day. ");
        }

        // 2. Process Comfort Preferences (Multipliers)
        let comfort_multiplier = match context.comfort_preference {
            ComfortLevel::VeryDim => 0.5,
            ComfortLevel::Dim => 0.75,
            ComfortLevel::Balanced => 1.0,
            ComfortLevel::Bright => 1.25,
            ComfortLevel::VeryBright => 1.5,
        };
        target_brightness *= comfort_multiplier;
        reasoning.push_str("Applied comfort multiplier. ");

        // 3. Process User Override
        if let Some(pref) = context.user_brightness_preference {
            confidence = 1.0; // Absolute confidence on user override
            target_brightness = pref as f32;
            reasoning.push_str("User preference overridden. ");
        }

        // Validate final value
        let final_brightness = target_brightness.round().clamp(0.0, 100.0) as u8;

        Ok(DecisionResult {
            recommended_brightness: final_brightness,
            confidence: confidence.clamp(0.0, 1.0),
            reasoning: reasoning.trim().to_string(),
        })
    }
}
