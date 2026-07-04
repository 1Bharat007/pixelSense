use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientReading, SensorInfo};
use crate::ambient::provider::AmbientProvider;
use std::sync::Arc;

pub struct SensorRegistry {
    providers: Vec<Arc<dyn AmbientProvider>>,
    infos: Vec<SensorInfo>,
}

impl SensorRegistry {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            infos: Vec::new(),
        }
    }

    pub fn register(&mut self, provider: Arc<dyn AmbientProvider>) {
        if let Ok(info) = provider.initialize() {
            self.providers.push(provider);
            self.infos.push(info);
        } else {
            // Log failure, but do not panic.
            log::warn!("Failed to initialize sensor: {}", provider.get_sensor_id());
        }
    }

    pub fn get_infos(&self) -> &[SensorInfo] {
        &self.infos
    }

    /// Read from all registered sensors.
    /// In the future, this could average readings, but for now it returns the primary valid reading.
    pub fn read_primary(&self) -> Result<AmbientReading, AmbientError> {
        for provider in &self.providers {
            if let Ok(reading) = provider.read_ambient_light() {
                return Ok(reading);
            }
        }
        Err(AmbientError::SensorUnavailable("No sensors available in registry".into()))
    }

    pub fn suspend_all(&self) {
        for provider in &self.providers {
            provider.suspend();
        }
    }

    pub fn resume_all(&self) {
        for provider in &self.providers {
            provider.resume();
        }
    }
}

impl Default for SensorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
