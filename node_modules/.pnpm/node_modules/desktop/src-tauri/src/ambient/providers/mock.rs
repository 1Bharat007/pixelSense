use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientEnvironment, AmbientQuality, AmbientReading, AmbientSensorType};
use crate::ambient::provider::AmbientProvider;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MockAmbientProvider {
    pub current_lux: Mutex<f32>,
    pub sensor_id: String,
    pub is_available: Mutex<bool>,
}

impl MockAmbientProvider {
    pub fn new(id: String) -> Self {
        Self {
            current_lux: Mutex::new(150.0), // default indoor
            sensor_id: id,
            is_available: Mutex::new(true),
        }
    }
    
    pub fn set_lux(&self, lux: f32) {
        *self.current_lux.lock().unwrap() = lux;
    }
    
    pub fn set_available(&self, available: bool) {
        *self.is_available.lock().unwrap() = available;
    }
}

impl AmbientProvider for MockAmbientProvider {
    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        let is_available = *self.is_available.lock().unwrap();
        if !is_available {
            return Err(AmbientError::SensorUnavailable("Mock sensor disabled".into()));
        }

        let lux = *self.current_lux.lock().unwrap();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;

        Ok(AmbientReading {
            source_id: self.sensor_id.clone(),
            lux,
            normalized_lux: lux, // Pre-smoothing
            environment: AmbientReading::determine_environment(lux),
            confidence: 1.0,
            sensor_type: AmbientSensorType::NativeSensor,
            timestamp,
            quality: AmbientQuality::Excellent,
            is_stable: true,
        })
    }

    fn get_sensor_id(&self) -> String {
        self.sensor_id.clone()
    }
}
