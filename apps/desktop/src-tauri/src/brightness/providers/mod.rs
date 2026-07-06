use crate::brightness::error::BrightnessError;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub mod windows;
pub mod mock;

pub trait BrightnessProvider: Send + Sync {
    fn set_brightness(
        &self,
        display: &DisplayInfo,
        capabilities: &DisplayCapabilities,
        level: u8,
    ) -> Result<(), BrightnessError>;

    fn get_brightness(&self, display: &DisplayInfo) -> Result<u8, BrightnessError>;
}
