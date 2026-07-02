use crate::capabilities::error::CapabilityError;
use crate::capabilities::providers::CapabilityProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub struct LinuxCapabilityProvider;

impl LinuxCapabilityProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinuxCapabilityProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityProvider for LinuxCapabilityProvider {
    fn evaluate(&self, _display: &DisplayInfo) -> Result<DisplayCapabilities, CapabilityError> {
        Err(CapabilityError::NotImplemented("Linux capability provider placeholder".into()))
    }
}
