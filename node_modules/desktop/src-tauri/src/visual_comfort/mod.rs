pub mod engine;
pub mod factory;
pub mod filters;
pub mod models;
pub mod strategies;

#[cfg(test)]
mod tests {
    use crate::visual_comfort::factory::create_visual_comfort_engine;
    use crate::visual_comfort::models::{ComfortConfig, VisualComfortContext, RecommendationAction, ComfortProfile};
    use std::thread;
    use std::time::Duration;

    fn mock_context() -> VisualComfortContext {
        VisualComfortContext {
            display_id: "disp_1".into(),
            current_comfort_profile: Some(ComfortProfile {
                profile_id: "1".into(),
                profile_name: "Test".into(),
                display_identifier: "disp_1".into(),
                ambient_light: 100.0,
                average_screen_luminance: 50.0, // locked luminance
                monitor_brightness: 50,       // locked brightness
                comfort_timestamp: 0,
                calibration_quality: 1.0,
                schema_version: 1,
                algorithm_version: 1,
            }),
            ambient_light: Some(100.0),
            screen_luminance: Some(100.0), // current luminance (doubled!)
            current_monitor_brightness: 50,
            predicted_emitted_light: 5000.0,
            time_of_day: "Day".into(),
            transition_enabled: true,
            confidence: 1.0,
        }
    }

    #[test]
    fn test_compensation_dark_to_bright() {
        let engine = create_visual_comfort_engine(ComfortConfig::default());
        let ctx = mock_context();
        
        let result = engine.calculate_comfort(ctx);
        // Luminance doubled (50 -> 100).
        // Brightness should halve (50 -> 25) to compensate.
        assert_eq!(result.recommendation.recommended_brightness, 25);
        assert_eq!(result.recommendation.action, RecommendationAction::SmoothTransition);
    }

    #[test]
    fn test_compensation_bright_to_dark() {
        let engine = create_visual_comfort_engine(ComfortConfig::default());
        let mut ctx = mock_context();
        ctx.screen_luminance = Some(25.0); // Halved
        
        let result = engine.calculate_comfort(ctx);
        // Brightness should double (50 -> 100).
        assert_eq!(result.recommendation.recommended_brightness, 100);
    }

    #[test]
    fn test_threshold_filtering() {
        let engine = create_visual_comfort_engine(ComfortConfig {
            minimum_change_threshold: 10, // high threshold
            ..Default::default()
        });
        
        let mut ctx = mock_context();
        ctx.screen_luminance = Some(45.0); // Minor change
        
        let result = engine.calculate_comfort(ctx);
        assert_eq!(result.recommendation.action, RecommendationAction::Ignore);
    }

    #[test]
    fn test_rate_limiting() {
        let engine = create_visual_comfort_engine(ComfortConfig {
            minimum_update_interval: 500,
            ..Default::default()
        });
        
        let ctx1 = mock_context();
        let result1 = engine.calculate_comfort(ctx1.clone());
        assert_ne!(result1.recommendation.action, RecommendationAction::Ignore);
        
        let result2 = engine.calculate_comfort(ctx1.clone());
        // Should be rate limited immediately after
        assert_eq!(result2.recommendation.action, RecommendationAction::Ignore);
        
        thread::sleep(Duration::from_millis(600));
        
        let result3 = engine.calculate_comfort(ctx1.clone());
        // Should pass after interval
        assert_ne!(result3.recommendation.action, RecommendationAction::Ignore);
    }

    #[test]
    fn test_missing_profile() {
        let engine = create_visual_comfort_engine(ComfortConfig::default());
        let mut ctx = mock_context();
        ctx.current_comfort_profile = None;
        
        let result = engine.calculate_comfort(ctx);
        assert_eq!(result.recommendation.action, RecommendationAction::NoChange);
    }

    #[test]
    fn property_test_brightness_bounds() {
        let engine = create_visual_comfort_engine(ComfortConfig {
            minimum_brightness: 10,
            maximum_brightness: 100,
            ..Default::default()
        });

        // Simulating property-based testing across thousands of bounds
        for lux in 0..10000 {
            for luminance in 0..500 {
                let mut ctx = mock_context();
                ctx.ambient_light = Some(lux as f32);
                ctx.screen_luminance = Some(luminance as f32);
                
                let result = engine.calculate_comfort(ctx);
                
                if result.recommendation.action != RecommendationAction::Ignore && result.recommendation.action != RecommendationAction::NoChange {
                    let b = result.recommendation.recommended_brightness;
                    assert!(b >= 10 && b <= 100, "Property violated: brightness {} is out of bounds [10, 100]", b);
                }
            }
        }
    }
}
