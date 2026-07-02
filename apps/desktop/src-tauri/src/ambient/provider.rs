use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientReading, SensorInfo};

pub trait AmbientProvider: Send + Sync {
    /// Initialize the provider (e.g., set up COM callbacks).
    fn initialize(&self) -> Result<SensorInfo, AmbientError>;
    
    /// Returns the cached ambient reading. Non-blocking.
    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError>;
    
    fn get_sensor_id(&self) -> String;
    
    /// Suspends callbacks/polling (e.g., during OS sleep).
    fn suspend(&self) {}
    
    /// Resumes callbacks/polling.
    fn resume(&self) {}
}
