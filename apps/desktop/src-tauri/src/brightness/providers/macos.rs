use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub struct MacOSBrightnessProvider;

impl MacOSBrightnessProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacOSBrightnessProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl BrightnessProvider for MacOSBrightnessProvider {
    fn set_brightness(
        &self,
        _display: &DisplayInfo,
        _capabilities: &DisplayCapabilities,
        _brightness_percent: u8,
    ) -> Result<(), BrightnessError> {
        Err(BrightnessError::NotImplemented("macOS brightness not implemented".into()))
    }
}
