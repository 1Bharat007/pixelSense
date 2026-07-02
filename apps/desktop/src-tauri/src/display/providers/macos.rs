use crate::display::domain::{DisplayError, DisplayInfo};
use super::DisplayProvider;

pub struct MacOSProvider;

impl MacOSProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacOSProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayProvider for MacOSProvider {
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, DisplayError> {
        // Placeholder for macOS-specific implementation
        Ok(vec![])
    }
}
