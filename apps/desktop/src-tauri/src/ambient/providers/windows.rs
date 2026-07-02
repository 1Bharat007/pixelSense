use crate::ambient::error::AmbientError;
use crate::ambient::models::AmbientReading;
use crate::ambient::provider::AmbientProvider;

pub struct WindowsAmbientProvider;

impl WindowsAmbientProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowsAmbientProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AmbientProvider for WindowsAmbientProvider {
    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        Err(AmbientError::SensorUnavailable("Windows native sensor API not implemented".into()))
    }
    
    fn get_sensor_id(&self) -> String {
        "windows_native".into()
    }
}
