pub mod linux;
pub mod macos;
pub mod mock;
pub mod windows;

use crate::capabilities::error::CapabilityError;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub trait CapabilityProvider: Send + Sync {
    fn evaluate(&self, display: &DisplayInfo) -> Result<DisplayCapabilities, CapabilityError>;
}
