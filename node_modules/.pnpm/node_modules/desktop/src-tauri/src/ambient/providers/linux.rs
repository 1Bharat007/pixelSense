use crate::ambient::error::AmbientError;
use crate::ambient::models::AmbientReading;
use crate::ambient::provider::AmbientProvider;

pub struct LinuxAmbientProvider;

impl LinuxAmbientProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinuxAmbientProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AmbientProvider for LinuxAmbientProvider {
    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        Err(AmbientError::SensorUnavailable("Linux native sensor API not implemented".into()))
    }
    
    fn get_sensor_id(&self) -> String {
        "Linux_native".into()
    }
}
