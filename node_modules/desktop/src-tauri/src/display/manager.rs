use crate::display::domain::{DisplayError, DisplayInfo};
use crate::display::providers::DisplayProvider;

pub struct DisplayManager {
    provider: Box<dyn DisplayProvider>,
}

impl DisplayManager {
    pub fn new(provider: Box<dyn DisplayProvider>) -> Self {
        Self { provider }
    }

    pub fn get_displays(&self) -> Result<Vec<DisplayInfo>, DisplayError> {
        // Future extension: caching and validation can be implemented here.
        self.provider.get_displays()
    }
}
