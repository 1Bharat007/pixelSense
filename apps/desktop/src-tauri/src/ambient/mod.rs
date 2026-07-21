pub mod calibration;
pub mod confidence;
pub mod config;
pub mod error;
pub mod manager;
pub mod models;
pub mod provider;
pub mod registry;
pub mod smoothing;

#[cfg(any())]
mod tests {
    use crate::ambient::calibration::linear::LinearCalibration;
    use crate::ambient::config::AmbientConfig;
    use crate::ambient::error::AmbientError;
    use crate::ambient::manager::AmbientManager;
    use crate::ambient::models::{AmbientEnvironment, AmbientSensorType, SensorState};
    use crate::ambient::registry::SensorRegistry;
    use crate::ambient::smoothing::BasicSmoothingStrategy;
    use std::sync::Arc;

    fn create_test_manager(provider: Arc<MockAmbientProvider>, config: Option<AmbientConfig>) -> AmbientManager {
        let mut registry = SensorRegistry::new();
        registry.register(provider);
        AmbientManager::new(
            config.unwrap_or_default(),
            registry,
            Box::new(LinearCalibration::new(10000.0)),
            Box::new(BasicSmoothingStrategy::new(5)),
        )
    }

    #[test]
    fn test_environment_mapping() {
        let provider = Arc::new(MockAmbientProvider::new());
        provider.set_lux(150.0);
        let manager = create_test_manager(provider, None);

        let reading = manager.get_ambient_light().unwrap();
        assert_eq!(reading.environment, AmbientEnvironment::Indoor);
        assert_eq!(reading.is_estimated, false);
    }

    #[test]
    fn test_sensor_unavailable_fallback_policy() {
        let provider = Arc::new(MockAmbientProvider::new());
        provider.set_available(false); // Disable sensor
        
        let manager = create_test_manager(provider, None);
        
        let result = manager.get_ambient_light();
        assert!(result.is_ok());
        let reading = result.unwrap();
        
        // Assert fallback policy matches
        assert_eq!(reading.sensor_type, AmbientSensorType::EstimatedUnavailable);
        assert_eq!(reading.confidence, 0.0);
        assert_eq!(reading.is_estimated, true);
        
        let health = manager.get_health();
        assert_eq!(health.current_state, SensorState::Unavailable);
    }

    #[test]
    fn test_threshold_filtering() {
        let provider = Arc::new(MockAmbientProvider::new());
        provider.set_lux(100.0);
        
        let mut config = AmbientConfig::default();
        config.minimum_change_threshold = 10.0;
        config.smoothing_enabled = false;
        
        let mut registry = SensorRegistry::new();
        registry.register(provider.clone());
        
        let manager = AmbientManager::new(
            config,
            registry,
            Box::new(LinearCalibration::new(10000.0)),
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
        let provider = Arc::new(MockAmbientProvider::new());
        
        let mut config = AmbientConfig::default();
        config.minimum_change_threshold = 0.0; // Disable threshold
        config.smoothing_enabled = true;
        
        let mut registry = SensorRegistry::new();
        registry.register(provider.clone());
        
        let manager = AmbientManager::new(
            config,
            registry,
            Box::new(LinearCalibration::new(10000.0)),
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
        assert_eq!(r3.normalized_lux, 250.0); // (200 + 300) / 2
    }
}
