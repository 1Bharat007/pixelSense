use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub struct WindowsBrightnessProvider {}

impl WindowsBrightnessProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl BrightnessProvider for WindowsBrightnessProvider {
    fn set_brightness(
        &self,
        _display: &DisplayInfo,
        _capabilities: &DisplayCapabilities,
        _level: u8,
    ) -> Result<(), BrightnessError> {
        // TODO: Implement WmiMonitorBrightness or DDC/CI
        Ok(())
    }

    fn get_brightness(&self, _display: &DisplayInfo) -> Result<u8, BrightnessError> {
        Ok(50)
    }
}
