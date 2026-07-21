use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientReading, SensorInfo, AmbientSensorType, AmbientQuality};
use crate::ambient::provider::AmbientProvider;
use crate::platform::hardware::sensor::manager::SensorSession;
use std::sync::Mutex;
use crate::background::models::now_ms;

pub struct NativeSensorProvider {
    session: Mutex<SensorSession>,
}

impl NativeSensorProvider {
    pub fn new() -> Self {
        Self {
            session: Mutex::new(SensorSession::new()),
        }
    }
}

impl AmbientProvider for NativeSensorProvider {
    fn initialize(&self) -> Result<SensorInfo, AmbientError> {
        let _ = self.session.lock().unwrap().read_lux(); // Just to test
        Ok(SensorInfo {
            manufacturer: "Windows".into(),
            device_name: "Windows Sensor API".into(),
            hardware_id: "N/A".into(),
            driver_version: "1.0".into(),
            supports_events: false,
            supports_polling: true,
            minimum_lux: 0.0,
            maximum_lux: 100000.0,
            sampling_frequency: 1000,
        })
    }
    
    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        let lux = self.session.lock().unwrap().read_lux().map_err(|e| AmbientError::ReadFailed(e.to_string()))?;
        Ok(AmbientReading {
            source_id: "windows_sensor".into(),
            sensor_name: "Windows Sensor API".into(),
            lux,
            normalized_lux: lux,
            environment: AmbientReading::from_lux(lux),
            confidence: 0.9,
            sensor_type: AmbientSensorType::NativeSensor,
            timestamp: now_ms(),
            quality: AmbientQuality::Good,
            is_stable: true,
            reading_duration_ms: 10,
            is_estimated: false,
        })
    }
    
    fn get_sensor_id(&self) -> String {
        "windows_sensor".into()
    }
}
