use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientEnvironment, AmbientQuality, AmbientReading, AmbientSensorType, SensorInfo};
use crate::ambient::provider::AmbientProvider;
use crate::background::models::now_ms;
use std::sync::Mutex;

pub struct MockAmbientProvider {
    pub is_available: Mutex<bool>,
    pub mock_lux: Mutex<f32>,
}

impl MockAmbientProvider {
    pub fn new() -> Self {
        Self {
            is_available: Mutex::new(true),
            mock_lux: Mutex::new(250.0),
        }
    }
    
    pub fn set_lux(&self, lux: f32) {
        *self.mock_lux.lock().unwrap() = lux;
    }
    
    pub fn set_available(&self, available: bool) {
        *self.is_available.lock().unwrap() = available;
    }
}

impl AmbientProvider for MockAmbientProvider {
    fn initialize(&self) -> Result<SensorInfo, AmbientError> {
        if !*self.is_available.lock().unwrap() {
            return Err(AmbientError::SensorUnavailable("Mock sensor disabled".into()));
        }
        
        Ok(SensorInfo {
            manufacturer: "Mockingbird".into(),
            device_name: "Mock Sensor".into(),
            hardware_id: "MOCK_001".into(),
            driver_version: "1.0".into(),
            supports_events: false,
            supports_polling: true,
            minimum_lux: 0.0,
            maximum_lux: 10000.0,
            sampling_frequency: 100,
        })
    }

    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        if !*self.is_available.lock().unwrap() {
            return Err(AmbientError::SensorUnavailable("Mock sensor disabled".into()));
        }
        
        let lux = *self.mock_lux.lock().unwrap();

        Ok(AmbientReading {
            source_id: self.get_sensor_id(),
            sensor_name: "Mock Sensor".into(),
            lux,
            normalized_lux: 0.0,
            environment: AmbientReading::determine_environment(lux),
            confidence: 1.0,
            sensor_type: AmbientSensorType::NativeSensor,
            timestamp: now_ms(),
            quality: AmbientQuality::Good,
            is_stable: true,
            reading_duration_ms: 1,
            is_estimated: false,
        })
    }

    fn get_sensor_id(&self) -> String {
        "mock_sensor".into()
    }
}
