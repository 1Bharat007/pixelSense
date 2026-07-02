use crate::ambient::config::AmbientConfig;
use crate::ambient::manager::AmbientManager;
use crate::ambient::providers::mock::MockAmbientProvider;
use crate::ambient::smoothing::BasicSmoothingStrategy;

pub fn create_ambient_manager(config: AmbientConfig) -> AmbientManager {
    // For Sprint 13, inject MockAmbientProvider as per requirements since Windows/Linux/Mac are placeholders.
    let provider = Box::new(MockAmbientProvider::new("mock_sensor_1".into()));
    let smoothing_strategy = Box::new(BasicSmoothingStrategy::new(5));

    AmbientManager::new(config, provider, smoothing_strategy)
}
