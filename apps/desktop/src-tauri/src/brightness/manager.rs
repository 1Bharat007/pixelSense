use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

/// BrightnessManager safely coordinates setting brightness.
///
/// **Future Responsibilities (TODO):**
/// - Expose `set_brightness_strict()` and `set_brightness_clamped()` explicitly.
/// - Implement brightness history.
/// - Support smooth transitions.
/// - Support undo operations.
/// - Support temporary dimming.
/// - Thread safety considerations when tracking history and states.
pub struct BrightnessManager {
    provider: Box<dyn BrightnessProvider>,
}

impl BrightnessManager {
    pub fn new(provider: Box<dyn BrightnessProvider>) -> Self {
        Self { provider }
    }

    /// Sets the brightness of a display.
    /// Clamps the value between 0 and 100.
    pub fn set_brightness(
        &self,
        display: &DisplayInfo,
        capabilities: &DisplayCapabilities,
        brightness_percent: i32, // Accept wider type to demonstrate clamping and validation
    ) -> Result<(), BrightnessError> {
        if !capabilities.brightness {
            return Err(BrightnessError::UnsupportedDisplay(
                "Display does not support brightness control".into(),
            ));
        }

        // Clamp brightness between 0 and 100
        let clamped = brightness_percent.clamp(0, 100) as u8;

        self.provider
            .set_brightness(display, capabilities, clamped)
    }
    
    pub fn get_brightness(&self, display: &DisplayInfo) -> Result<u8, BrightnessError> {
        self.provider.get_brightness(display)
    }
}
