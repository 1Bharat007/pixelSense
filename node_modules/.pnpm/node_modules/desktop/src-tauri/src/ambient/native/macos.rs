use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientReading, SensorInfo};
use crate::ambient::provider::AmbientProvider;

pub struct MacosAmbientProvider;

impl AmbientProvider for MacosAmbientProvider {
    fn initialize(&self) -> Result<SensorInfo, AmbientError> {
        Err(AmbientError::NotSupported("macOS ALS via IOKit not yet implemented".into()))
    }

    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        Err(AmbientError::SensorUnavailable("Not supported".into()))
    }

    fn get_sensor_id(&self) -> String {
        "macos_stub".into()
    }
}
