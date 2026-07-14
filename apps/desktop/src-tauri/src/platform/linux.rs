use super::{Platform, PlatformError};
use crate::display::domain::DisplayInfo;
use crate::platform::capabilities::PlatformCapabilities;

pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinuxPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Platform for LinuxPlatform {
    fn get_capabilities(&self) -> Result<PlatformCapabilities, PlatformError> {
        Ok(PlatformCapabilities::default())
    }

    fn discover_displays(&self) -> Result<Vec<DisplayInfo>, PlatformError> {
        Err(PlatformError::NotImplemented("Linux display discovery not implemented".into()))
    }

    fn discover_capabilities(&self, _display: &DisplayInfo) -> Result<crate::display::domain::DisplayCapabilities, PlatformError> {
        Err(PlatformError::NotImplemented("Linux capability placeholder".into()))
    }

    fn set_brightness(&self, _display: &DisplayInfo, _brightness_percent: u8) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented("Linux capability discovery not implemented".into()))
    }

    fn get_config_path(&self) -> Result<String, PlatformError> {
        Err(PlatformError::NotImplemented("Linux config path not implemented".into()))
    }

    fn send_notification(&self) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented("Linux notifications not implemented".into()))
    }
}


