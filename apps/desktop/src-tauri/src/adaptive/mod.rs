pub mod config;
pub mod error;
pub mod factory;
pub mod service;
pub mod state;

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::adaptive::config::AdaptiveConfig;
    use crate::adaptive::error::AdaptiveError;
    use crate::adaptive::service::AdaptiveBrightnessService;
    use crate::adaptive::state::BrightnessState;
    use crate::brightness::manager::BrightnessManager;
    use crate::brightness::providers::mock::MockBrightnessProvider;
    use crate::decision::config::DecisionConfig;
    use crate::decision::manager::DecisionManager;
    use crate::decision::models::{AmbientLightReading, ComfortLevel, DecisionContext, TimeOfDay};
    use crate::decision::strategies::default::DefaultDecisionStrategy;
    use crate::display::domain::{DisplayCapabilities, DisplayInfo};
    use crate::transition::config::TransitionConfig;
    use crate::transition::manager::TransitionManager;
    use crate::transition::providers::mock::MockTransitionProvider;

    fn create_dummy_display() -> (DisplayInfo, DisplayCapabilities) {
        let caps = DisplayCapabilities { brightness: true, hdr: false, ddc_ci: false };
        let display = DisplayInfo {
            id: "laptop_id".into(),
            name: "Laptop".into(),
            manufacturer: None,
            model: None,
            width: 1920,
            height: 1080,
            refresh_rate: None,
            is_primary: true,
            capabilities: caps.clone(),
        };
        (display, caps)
    }

    fn setup(config: AdaptiveConfig) -> (AdaptiveBrightnessService, Arc<Mutex<BrightnessState>>, MockTransitionProvider) {
        let brightness_provider = Box::new(MockBrightnessProvider::new());
        let brightness_manager = Arc::new(BrightnessManager::new(brightness_provider));
        
        let decision_manager = DecisionManager::new(Box::new(DefaultDecisionStrategy::new()), DecisionConfig::default());
        
        let mock_transition = MockTransitionProvider::new();
        let transition_manager = TransitionManager::new(
            Box::new(mock_transition.clone()),
            Arc::clone(&brightness_manager),
            TransitionConfig::default(),
        );

        let state = Arc::new(Mutex::new(BrightnessState::new()));
        let service = AdaptiveBrightnessService::new(decision_manager, transition_manager, config, Arc::clone(&state));

        (service, state, mock_transition)
    }

    #[test]
    fn test_successful_adaptive_flow() {
        let (service, state, mock_trans) = setup(AdaptiveConfig::default());
        let (display, caps) = create_dummy_display();

        let ctx = DecisionContext {
            ambient_light: Some(AmbientLightReading { lux: 1500.0 }), // Expect 90 brightness
            user_brightness_preference: None,
            comfort_preference: ComfortLevel::Balanced,
            time_of_day: TimeOfDay::Day,
        };

        service.execute_pipeline(&display, &caps, &ctx).unwrap();

        // Verify state is updated
        assert_eq!(state.lock().unwrap().get_brightness(&display.id), 90);

        // Verify transition mock received the steps
        let records = mock_trans.records.lock().unwrap();
        assert!(!records.is_empty());
        assert_eq!(records.last().unwrap().brightness, 90);
    }

    #[test]
    fn test_adaptive_disabled() {
        let mut config = AdaptiveConfig::default();
        config.adaptive_enabled = false;
        let (service, _state, _mock) = setup(config);
        let (display, caps) = create_dummy_display();

        let ctx = DecisionContext {
            ambient_light: None,
            user_brightness_preference: None,
            comfort_preference: ComfortLevel::Balanced,
            time_of_day: TimeOfDay::Day,
        };

        let result = service.execute_pipeline(&display, &caps, &ctx);
        assert!(matches!(result, Err(AdaptiveError::AdaptiveDisabled)));
    }

    #[test]
    fn test_confidence_below_threshold() {
        let mut config = AdaptiveConfig::default();
        config.confidence_threshold = 0.9; // Require high confidence
        let (service, _state, _mock) = setup(config);
        let (display, caps) = create_dummy_display();

        let ctx = DecisionContext {
            ambient_light: None, // Missing sensor data = 0.5 confidence
            user_brightness_preference: None,
            comfort_preference: ComfortLevel::Balanced,
            time_of_day: TimeOfDay::Day,
        };

        let result = service.execute_pipeline(&display, &caps, &ctx);
        assert!(matches!(result, Err(AdaptiveError::ConfidenceTooLow(_))));
    }

    #[test]
    fn test_transition_disabled_immediate_execution() {
        let mut config = AdaptiveConfig::default();
        config.transition_enabled = false;
        let (service, state, mock_trans) = setup(config);
        let (display, caps) = create_dummy_display();

        let ctx = DecisionContext {
            ambient_light: Some(AmbientLightReading { lux: 1500.0 }), 
            user_brightness_preference: None,
            comfort_preference: ComfortLevel::Balanced,
            time_of_day: TimeOfDay::Day,
        };

        service.execute_pipeline(&display, &caps, &ctx).unwrap();

        assert_eq!(state.lock().unwrap().get_brightness(&display.id), 90);

        // Immediate mode means 0 duration, so exactly 1 step
        let records = mock_trans.records.lock().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].brightness, 90);
    }

    #[test]
    fn test_manual_override_placeholder() {
        let (service, _state, _mock) = setup(AdaptiveConfig::default());
        let (display, caps) = create_dummy_display();

        // User preference = absolute confidence (1.0)
        let ctx = DecisionContext {
            ambient_light: None,
            user_brightness_preference: Some(75),
            comfort_preference: ComfortLevel::Balanced,
            time_of_day: TimeOfDay::Day,
        };

        let result = service.execute_pipeline(&display, &caps, &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decision_failure() {
        // Our DefaultDecisionStrategy never fails currently, but if we pass an invalid
        // context that hypothetically failed, we'd get a wrapped error.
        // For testing, since we can't easily force it to fail without modifying the strategy,
        // we acknowledge that `DecisionError` correctly converts `Into<AdaptiveError>`.
        let err: AdaptiveError = crate::decision::error::DecisionError::CalculationFailed("test".into()).into();
        assert!(matches!(err, AdaptiveError::DecisionFailed(_)));
    }

    #[test]
    fn test_transition_failure() {
        let err: AdaptiveError = crate::transition::error::TransitionError::InvalidDuration("test".into()).into();
        assert!(matches!(err, AdaptiveError::TransitionFailed(_)));
    }
}
