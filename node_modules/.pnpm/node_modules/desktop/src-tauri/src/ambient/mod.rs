pub mod confidence;
pub mod config;
pub mod error;
pub mod factory;
pub mod manager;
pub mod models;
pub mod provider;
pub mod providers;
pub mod smoothing;

#[cfg(test)]
mod tests {
    use crate::ambient::config::AmbientConfig;
    use crate::ambient::error::AmbientError;
    use crate::ambient::factory::create_ambient_manager;
    use crate::ambient::models::{AmbientEnvironment, AmbientSensorState};
    use crate::ambient::providers::mock::MockAmbientProvider;

    #[test]
    fn test_environment_mapping() {
        let manager = create_ambient_manager(AmbientConfig::default());
        // Since we inject mock provider via factory, we can't easily change lux directly 
        // without downcasting, so we'll just test the mock default (150 lux -> Indoor)
        let reading = manager.get_ambient_light().unwrap();
        assert_eq!(reading.environment, AmbientEnvironment::Indoor);
    }

    #[test]
    fn test_sensor_unavailable() {
        use crate::ambient::manager::AmbientManager;
        use crate::ambient::smoothing::BasicSmoothingStrategy;
        
        let provider = Box::new(MockAmbientProvider::new("mock_2".into()));
        provider.set_available(false); // Disable sensor
        
        let manager = AmbientManager::new(
            AmbientConfig::default(),
            provider,
            Box::new(BasicSmoothingStrategy::new(5)),
        );
        
        let result = manager.get_ambient_light();
        assert!(matches!(result, Err(AmbientError::SensorUnavailable(_))));
        assert_eq!(manager.get_state(), AmbientSensorState::Error);
    }

    #[test]
    fn test_threshold_filtering() {
        use crate::ambient::manager::AmbientManager;
        use crate::ambient::smoothing::BasicSmoothingStrategy;

        let provider = Box::new(MockAmbientProvider::new("mock_3".into()));
        provider.set_lux(100.0);
        
        let manager = AmbientManager::new(
            AmbientConfig {
                minimum_change_threshold: 10.0,
                smoothing_enabled: false,
                ..Default::default()
            },
            provider.clone(),
            Box::new(BasicSmoothingStrategy::new(1)),
        );

        let reading1 = manager.get_ambient_light().unwrap();
        assert_eq!(reading1.normalized_lux, 100.0);
        assert_eq!(reading1.is_stable, true); // First reading is considered stable

        provider.set_lux(105.0); // Below 10.0 threshold
        let reading2 = manager.get_ambient_light().unwrap();
        assert_eq!(reading2.normalized_lux, 100.0); // Should remain 100.0
        assert_eq!(reading2.is_stable, true);

        provider.set_lux(120.0); // Above 10.0 threshold
        let reading3 = manager.get_ambient_light().unwrap();
        assert_eq!(reading3.normalized_lux, 120.0);
        assert_eq!(reading3.is_stable, false); // Just changed
    }

    #[test]
    fn test_smoothing() {
        use crate::ambient::manager::AmbientManager;
        use crate::ambient::smoothing::BasicSmoothingStrategy;

        let provider = Box::new(MockAmbientProvider::new("mock_4".into()));
        
        let manager = AmbientManager::new(
            AmbientConfig {
                minimum_change_threshold: 0.0, // Disable threshold to observe smoothing pure math
                smoothing_enabled: true,
                ..Default::default()
            },
            provider.clone(),
            Box::new(BasicSmoothingStrategy::new(2)), // Moving average of last 2
        );

        provider.set_lux(100.0);
        let r1 = manager.get_ambient_light().unwrap();
        assert_eq!(r1.normalized_lux, 100.0); // (100) / 1

        provider.set_lux(200.0);
        let r2 = manager.get_ambient_light().unwrap();
        assert_eq!(r2.normalized_lux, 150.0); // (100 + 200) / 2

        provider.set_lux(300.0);
        let r3 = manager.get_ambient_light().unwrap();
        assert_eq!(r3.normalized_lux, 250.0); // (200 + 300) / 2 (since max samples is 2)
    }
}
