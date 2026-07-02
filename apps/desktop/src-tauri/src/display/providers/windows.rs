use crate::display::domain::{DisplayError, DisplayInfo};
use super::DisplayProvider;
use crate::platform::factory::create_platform;

pub struct WindowsProvider;

impl WindowsProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayProvider for WindowsProvider {
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, DisplayError> {
        let platform = create_platform();
        platform.discover_displays().map_err(|e| DisplayError::PlatformError(e.to_string()))
    }
}
