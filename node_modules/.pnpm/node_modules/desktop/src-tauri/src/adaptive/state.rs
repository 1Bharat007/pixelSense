use std::collections::HashMap;

/// In-memory representation of the current brightness of displays.
/// Future implementations will synchronize this with the Platform APIs.
#[derive(Debug, Clone)]
pub struct BrightnessState {
    // Map of Display ID -> Current Brightness
    pub current_brightness: HashMap<String, u8>,
}

impl BrightnessState {
    pub fn new() -> Self {
        Self {
            current_brightness: HashMap::new(),
        }
    }

    pub fn get_brightness(&self, display_id: &str) -> u8 {
        *self.current_brightness.get(display_id).unwrap_or(&50) // Default fallback
    }

    pub fn update_brightness(&mut self, display_id: &str, brightness: u8) {
        self.current_brightness.insert(display_id.to_string(), brightness);
    }
}

impl Default for BrightnessState {
    fn default() -> Self {
        Self::new()
    }
}
