use crate::ambient::error::AmbientError;
use crate::ambient::models::AmbientReading;
use crate::ambient::provider::AmbientProvider;

pub struct MacosAmbientProvider;

impl MacosAmbientProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacosAmbientProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AmbientProvider for MacosAmbientProvider {
    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        Err(AmbientError::SensorUnavailable("Macos native sensor API not implemented".into()))
    }
    
    fn get_sensor_id(&self) -> String {
        "Macos_native".into()
    }
}
