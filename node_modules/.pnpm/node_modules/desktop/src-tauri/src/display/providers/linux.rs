use crate::display::domain::{DisplayError, DisplayInfo};
use super::DisplayProvider;

pub struct LinuxProvider;

impl LinuxProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinuxProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayProvider for LinuxProvider {
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, DisplayError> {
        // Placeholder for Linux-specific implementation
        Ok(vec![])
    }
}
