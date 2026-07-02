use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use std::sync::Mutex;
use crate::platform::factory::create_platform;
use crate::platform::facade::PlatformFacade; // Note: We'll assume create_platform() returns a struct that implements PlatformFacade, or we have a factory for the facade.

// For now, since `create_platform` returns `Box<dyn Platform>`, we need a way to cast or get the facade.
// In a full DI container this would be injected. We'll use a hack or just direct instantiation for Windows.
use crate::platform::windows::WindowsPlatform;

pub struct WindowsBrightnessProvider {
    last_brightness: Mutex<Option<(String, u8)>>,
    platform: WindowsPlatform,
}

impl WindowsBrightnessProvider {
    pub fn new() -> Self {
        Self {
            last_brightness: Mutex::new(None),
            platform: WindowsPlatform::new(),
        }
    }

    fn set_internal_brightness(&self, level: u8) -> Result<(), BrightnessError> {
        self.platform.brightness().set_internal_brightness(level)
            .map_err(|e| BrightnessError::PlatformFailure(e.to_string()))
    }

    fn set_external_brightness(&self, display: &DisplayInfo, level: u8) -> Result<(), BrightnessError> {
        self.platform.brightness().set_external_brightness(display, level)
            .map_err(|e| BrightnessError::PlatformFailure(e.to_string()))
    }
}

impl Default for WindowsBrightnessProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl BrightnessProvider for WindowsBrightnessProvider {
    fn set_brightness(
        &self,
        display: &DisplayInfo,
        _capabilities: &DisplayCapabilities,
        brightness_percent: u8,
    ) -> Result<(), BrightnessError> {
        // Brightness Cache: prevent duplicate hardware writes.
        {
            let mut cache = self.last_brightness.lock().unwrap();
            if let Some((cached_id, cached_level)) = &*cache {
                if cached_id == &display.id && *cached_level == brightness_percent {
                    log::debug!("Brightness write skipped (cache match): {} = {}%", display.id, brightness_percent);
                    return Ok(()); // Skipped
                }
            }
            *cache = Some((display.id.clone(), brightness_percent));
        }

        let name = display.name.to_lowercase();
        let is_internal = name.contains("internal") || name.contains("laptop") || display.is_primary;

        if is_internal {
            self.set_internal_brightness(brightness_percent)
        } else {
            self.set_external_brightness(display, brightness_percent)
        }
    }
}

