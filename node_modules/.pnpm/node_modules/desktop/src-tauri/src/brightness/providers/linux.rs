use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub struct LinuxBrightnessProvider;

impl LinuxBrightnessProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinuxBrightnessProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl BrightnessProvider for LinuxBrightnessProvider {
    fn set_brightness(
        &self,
        _display: &DisplayInfo,
        _capabilities: &DisplayCapabilities,
        _brightness_percent: u8,
    ) -> Result<(), BrightnessError> {
        Err(BrightnessError::NotImplemented("Linux brightness not implemented".into()))
    }
}
