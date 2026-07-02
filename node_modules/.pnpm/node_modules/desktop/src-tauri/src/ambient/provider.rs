use crate::ambient::error::AmbientError;
use crate::ambient::models::AmbientReading;

pub trait AmbientProvider: Send + Sync {
    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError>;
    fn get_sensor_id(&self) -> String;
}
