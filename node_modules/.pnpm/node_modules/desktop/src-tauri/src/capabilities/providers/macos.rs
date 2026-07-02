use crate::capabilities::error::CapabilityError;
use crate::capabilities::providers::CapabilityProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub struct MacOSCapabilityProvider;

impl MacOSCapabilityProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacOSCapabilityProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityProvider for MacOSCapabilityProvider {
    fn evaluate(&self, _display: &DisplayInfo) -> Result<DisplayCapabilities, CapabilityError> {
        Err(CapabilityError::NotImplemented("macOS capability provider placeholder".into()))
    }
}
