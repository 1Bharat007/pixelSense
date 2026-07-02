use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientEnvironment, AmbientQuality, AmbientReading, AmbientSensorType, SensorInfo};
use crate::ambient::provider::AmbientProvider;
use crate::background::models::now_ms;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use crate::platform::factory::create_platform;
use crate::platform::facade::PlatformFacade;
use crate::platform::windows::WindowsPlatform; // Using directly for now

/// Windows Native Ambient Light Sensor Provider
///
/// Uses the PlatformFacade to isolate the actual Windows Sensor API (`ISensorManager`) COM logic.
pub struct WindowsAmbientProvider {
    platform: WindowsPlatform,
}

impl WindowsAmbientProvider {
    pub fn new() -> Self {
        Self {
            platform: WindowsPlatform::new(),
        }
    }
}

impl AmbientProvider for WindowsAmbientProvider {
    fn initialize(&self) -> Result<SensorInfo, AmbientError> {
        // The real sensor initialization will be handled lazily or via events in PlatformFacade.
        Ok(SensorInfo {
            manufacturer: "Generic OEM".into(),
            device_name: "Windows Native ALS".into(),
            hardware_id: "ACPI\\ALS0001".into(),
            driver_version: "10.0".into(),
            supports_events: true,
            supports_polling: false,
            minimum_lux: 0.0,
            maximum_lux: 10000.0,
            sampling_frequency: 200,
        })
    }

    fn read_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        // Delegate to the Windows platform facade.
        self.platform.sensor().read_ambient_light()
            .map_err(|e| AmbientError::SensorUnavailable(e.to_string()))
    }

    fn get_sensor_id(&self) -> String {
        "windows_native_als".into()
    }
    
    fn suspend(&self) {
        log::info!("WindowsAmbientProvider suspended");
    }
    
    fn resume(&self) {
        log::info!("WindowsAmbientProvider resumed");
    }
}
