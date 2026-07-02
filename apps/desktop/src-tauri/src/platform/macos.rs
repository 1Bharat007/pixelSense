use super::{Platform, PlatformError};
use crate::display::domain::DisplayInfo;

pub struct MacOSPlatform;

impl MacOSPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacOSPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Platform for MacOSPlatform {
    fn discover_displays(&self) -> Result<Vec<DisplayInfo>, PlatformError> {
        Err(PlatformError::NotImplemented("macOS display discovery not implemented".into()))
    }

    fn discover_capabilities(&self, _display: &DisplayInfo) -> Result<crate::display::domain::DisplayCapabilities, PlatformError> {
        Err(PlatformError::NotImplemented("macOS capability placeholder".into()))
    }

    fn set_brightness(&self, _display: &DisplayInfo, _brightness_percent: u8) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented("macOS capability discovery not implemented".into()))
    }

    fn set_brightness(&self) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented("macOS brightness control not implemented".into()))
    }

    fn get_config_path(&self) -> Result<String, PlatformError> {
        Err(PlatformError::NotImplemented("macOS config path not implemented".into()))
    }

    fn send_notification(&self) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented("macOS notifications not implemented".into()))
    }
}


