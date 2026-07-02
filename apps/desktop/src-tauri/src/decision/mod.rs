pub mod config;
pub mod error;
pub mod factory;
pub mod manager;
pub mod models;
pub mod strategies;

#[cfg(test)]
mod tests {
    use crate::decision::config::DecisionConfig;
    use crate::decision::manager::DecisionManager;
    use crate::decision::models::{AmbientLightReading, ComfortLevel, DecisionContext, TimeOfDay};
    use crate::decision::strategies::default::DefaultDecisionStrategy;

    fn setup() -> DecisionManager {
        DecisionManager::new(Box::new(DefaultDecisionStrategy::new()), DecisionConfig::default())
    }

    #[test]
    fn test_bright_room() {
        let manager = setup();
        let ctx = DecisionContext {
            ambient_light: Some(AmbientLightReading { lux: 1500.0 }), // Bright
            user_brightness_preference: None,
            comfort_preference: ComfortLevel::Balanced,
            time_of_day: TimeOfDay::Day,
        };
        let result = manager.decide_brightness(&ctx).unwrap();
        
        assert_eq!(result.recommended_brightness, 90);
        assert_eq!(result.confidence, 0.8);
        assert!(result.reasoning.contains("Bright room"));
    }

    #[test]
    fn test_dark_room() {
        let manager = setup();
        let ctx = DecisionContext {
            ambient_light: Some(AmbientLightReading { lux: 5.0 }), // Dark
            user_brightness_preference: None,
            comfort_preference: ComfortLevel::Balanced,
            time_of_day: TimeOfDay::Night,
        };
        let result = manager.decide_brightness(&ctx).unwrap();
        
        assert_eq!(result.recommended_brightness, 15);
        assert_eq!(result.confidence, 0.8);
        assert!(result.reasoning.contains("Dark room"));
    }

    #[test]
    fn test_medium_room_with_comfort() {
        let manager = setup();
        let ctx = DecisionContext {
            ambient_light: Some(AmbientLightReading { lux: 300.0 }), // Medium
            user_brightness_preference: None,
            comfort_preference: ComfortLevel::Bright, // 1.25x multiplier
            time_of_day: TimeOfDay::Day,
        };
        let result = manager.decide_brightness(&ctx).unwrap();
        
        // Medium base = 50. 50 * 1.25 = 62.5 -> round -> 63
        assert_eq!(result.recommended_brightness, 63);
        assert!(result.reasoning.contains("comfort multiplier"));
    }

    #[test]
    fn test_preference_override() {
        let manager = setup();
        let ctx = DecisionContext {
            ambient_light: Some(AmbientLightReading { lux: 5.0 }), // Dark
            user_brightness_preference: Some(100), // Override to 100
            comfort_preference: ComfortLevel::Balanced,
            time_of_day: TimeOfDay::Night,
        };
        let result = manager.decide_brightness(&ctx).unwrap();
        
        assert_eq!(result.recommended_brightness, 100);
        assert_eq!(result.confidence, 1.0);
        assert!(result.reasoning.contains("User preference overridden"));
    }

    #[test]
    fn test_missing_ambient_data() {
        let manager = setup();
        let ctx = DecisionContext {
            ambient_light: None, // No sensor
            user_brightness_preference: None,
            comfort_preference: ComfortLevel::Balanced,
            time_of_day: TimeOfDay::Evening, // Base 40
        };
        let result = manager.decide_brightness(&ctx).unwrap();
        
        assert_eq!(result.recommended_brightness, 40);
        assert_eq!(result.confidence, 0.5); // Fallback confidence
        assert!(result.reasoning.contains("Fallback to time-of-day"));
    }
    
    #[test]
    fn test_conflicting_inputs() {
        let manager = setup();
        let ctx = DecisionContext {
            ambient_light: Some(AmbientLightReading { lux: 2000.0 }), // Super bright
            user_brightness_preference: Some(10), // User wants very dim
            comfort_preference: ComfortLevel::VeryBright, // Wants bright
            time_of_day: TimeOfDay::Night, // Night time
        };
        let result = manager.decide_brightness(&ctx).unwrap();
        
        // User preference is absolute override
        assert_eq!(result.recommended_brightness, 10);
        assert_eq!(result.confidence, 1.0);
    }
}
