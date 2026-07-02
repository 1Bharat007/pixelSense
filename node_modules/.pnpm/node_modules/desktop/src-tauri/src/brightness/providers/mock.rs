use std::collections::HashMap;
use std::sync::Mutex;
use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

/// Deterministic mock brightness provider.
///
/// **Future Note**: May migrate to `DashMap` if concurrency requirements increase.
pub struct MockBrightnessProvider {
    states: Mutex<HashMap<String, u8>>,
}

impl MockBrightnessProvider {
    pub fn new() -> Self {
        Self {
            states: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_brightness(&self, display_id: &str) -> Option<u8> {
        let lock = self.states.lock().unwrap();
        lock.get(display_id).copied()
    }
}

impl Default for MockBrightnessProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl BrightnessProvider for MockBrightnessProvider {
    fn set_brightness(
        &self,
        display: &DisplayInfo,
        _capabilities: &DisplayCapabilities,
        brightness_percent: u8,
    ) -> Result<(), BrightnessError> {
        let mut lock = self.states.lock().unwrap();
        lock.insert(display.id.clone(), brightness_percent);
        Ok(())
    }
}
