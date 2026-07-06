use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

#[derive(Clone)]
pub struct MockBrightnessProvider {}

impl MockBrightnessProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl BrightnessProvider for MockBrightnessProvider {
    fn set_brightness(
        &self,
        _display: &DisplayInfo,
        _capabilities: &DisplayCapabilities,
        _level: u8,
    ) -> Result<(), BrightnessError> {
        Ok(())
    }

    fn get_brightness(&self, _display: &DisplayInfo) -> Result<u8, BrightnessError> {
        Ok(50)
    }
}
