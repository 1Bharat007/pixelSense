use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientReading, SensorInfo};
use crate::ambient::provider::AmbientProvider;

pub struct LinuxAmbientProvider;

impl AmbientProvider for LinuxAmbientProvider {
    fn initialize(&self) -> Result<SensorInfo, AmbientError> {
        Err(AmbientError::NotSupported("Linux ALS via iio-sensor-proxy not yet implemented".into()))
    }

    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        Err(AmbientError::SensorUnavailable("Not supported".into()))
    }

    fn get_sensor_id(&self) -> String {
        "linux_stub".into()
    }
}
