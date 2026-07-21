use crate::brightness::error::BrightnessError;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub mod mock;
pub mod native;

pub trait BrightnessProvider: Send + Sync {
    fn set_brightness(
        &self,
        display: &DisplayInfo,
        capabilities: &DisplayCapabilities,
        level: u8,
    ) -> Result<(), BrightnessError>;

    fn get_brightness(&self, display: &DisplayInfo) -> Result<u8, BrightnessError>;
}
