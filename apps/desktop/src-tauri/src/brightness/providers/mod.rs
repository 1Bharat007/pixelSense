pub mod linux;
pub mod macos;
pub mod mock;
pub mod windows;

use crate::brightness::error::BrightnessError;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub trait BrightnessProvider: Send + Sync {
    fn set_brightness(
        &self,
        display: &DisplayInfo,
        capabilities: &DisplayCapabilities,
        brightness_percent: u8,
    ) -> Result<(), BrightnessError>;
}
