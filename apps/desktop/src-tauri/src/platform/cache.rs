use crate::ambient::models::AmbientReading;
use crate::display::domain::DisplayInfo;
use std::collections::HashMap;
use std::sync::RwLock;

/// HardwareStateCache prevents redundant API calls by caching the current state
/// of hardware and OS environments.
pub struct HardwareStateCache {
    pub brightness_levels: RwLock<HashMap<String, u8>>, // display_id -> level
    pub displays: RwLock<Vec<DisplayInfo>>,
    pub ambient_reading: RwLock<Option<AmbientReading>>,
    pub active_window_exe: RwLock<Option<String>>,
    pub on_battery: RwLock<bool>,
}

impl HardwareStateCache {
    pub fn new() -> Self {
        Self {
            brightness_levels: RwLock::new(HashMap::new()),
            displays: RwLock::new(Vec::new()),
            ambient_reading: RwLock::new(None),
            active_window_exe: RwLock::new(None),
            on_battery: RwLock::new(false),
        }
    }
}

impl Default for HardwareStateCache {
    fn default() -> Self {
        Self::new()
    }
}
