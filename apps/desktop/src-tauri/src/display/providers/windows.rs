use crate::display::domain::{DisplayError, DisplayInfo};
use crate::display::providers::DisplayProvider;

pub struct WindowsProvider;

impl WindowsProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl DisplayProvider for WindowsProvider {
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, DisplayError> {
        Ok(vec![])
    }
}
