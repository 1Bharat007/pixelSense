use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use crate::platform::factory::create_platform;

pub struct WindowsBrightnessProvider;

impl WindowsBrightnessProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowsBrightnessProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl BrightnessProvider for WindowsBrightnessProvider {
    fn set_brightness(
        &self,
        display: &DisplayInfo,
        _capabilities: &DisplayCapabilities,
        brightness_percent: u8,
    ) -> Result<(), BrightnessError> {
        // Delegate to the platform abstraction layer
        let platform = create_platform();
        platform.set_brightness(display, brightness_percent)
            .map_err(|e| BrightnessError::PlatformFailure(e.to_string()))
    }
}
