pub mod error;
pub mod factory;
pub mod manager;
pub mod memory;
pub mod providers;

#[cfg(test)]
mod tests {
    use super::factory::create_brightness_manager;
    use crate::brightness::error::BrightnessError;
    use crate::display::domain::{DisplayCapabilities, DisplayInfo};

    fn create_dummy_display(name: &str, capabilities: DisplayCapabilities) -> DisplayInfo {
        DisplayInfo {
            id: format!("{}_id", name),
            name: name.to_string(),
            manufacturer: None,
            model: None,
            width: 1920,
            height: 1080,
            refresh_rate: None,
            is_primary: true,
            capabilities,
        }
    }

    #[test]
    fn test_set_brightness_0() {
        let manager = create_brightness_manager();
        let caps = DisplayCapabilities { brightness: true, hdr: false, ddc_ci: false };
        let display = create_dummy_display("Laptop", caps.clone());

        assert!(manager.set_brightness(&display, &caps, 0).is_ok());
    }

    #[test]
    fn test_set_brightness_50() {
        let manager = create_brightness_manager();
        let caps = DisplayCapabilities { brightness: true, hdr: false, ddc_ci: false };
        let display = create_dummy_display("Laptop", caps.clone());

        assert!(manager.set_brightness(&display, &caps, 50).is_ok());
    }

    #[test]
    fn test_set_brightness_100() {
        let manager = create_brightness_manager();
        let caps = DisplayCapabilities { brightness: true, hdr: false, ddc_ci: false };
        let display = create_dummy_display("Laptop", caps.clone());

        assert!(manager.set_brightness(&display, &caps, 100).is_ok());
    }

    #[test]
    fn test_set_brightness_out_of_range_clamped() {
        let manager = create_brightness_manager();
        let caps = DisplayCapabilities { brightness: true, hdr: false, ddc_ci: false };
        let display = create_dummy_display("Laptop", caps.clone());

        // Should clamp and succeed without panicking
        assert!(manager.set_brightness(&display, &caps, -10).is_ok());
        assert!(manager.set_brightness(&display, &caps, 200).is_ok());
    }

    #[test]
    fn test_set_brightness_unsupported_display() {
        let manager = create_brightness_manager();
        let caps = DisplayCapabilities { brightness: false, hdr: false, ddc_ci: false };
        let display = create_dummy_display("Projector", caps.clone());

        let result = manager.set_brightness(&display, &caps, 50);
        assert!(matches!(result, Err(BrightnessError::UnsupportedDisplay(_))));
    }

    #[test]
    fn test_multiple_consecutive_changes() {
        let manager = create_brightness_manager();
        let caps = DisplayCapabilities { brightness: true, hdr: false, ddc_ci: false };
        let display = create_dummy_display("Laptop", caps.clone());

        assert!(manager.set_brightness(&display, &caps, 10).is_ok());
        assert!(manager.set_brightness(&display, &caps, 20).is_ok());
        assert!(manager.set_brightness(&display, &caps, 30).is_ok());
    }
}
