use super::{Platform, PlatformError};
use crate::display::domain::DisplayInfo;
use crate::platform::capabilities::PlatformCapabilities;
use super::models::NativeDisplay;

pub struct MockPlatform {
    pub return_empty_displays: bool,
}

impl MockPlatform {
    pub fn new() -> Self {
        Self { return_empty_displays: false }
    }
}

impl Default for MockPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Platform for MockPlatform {
    fn get_capabilities(&self) -> Result<PlatformCapabilities, PlatformError> {
        Ok(PlatformCapabilities::default()) // Mocks return defaults (false for everything)
    }

    fn discover_displays(&self) -> Result<Vec<DisplayInfo>, PlatformError> {
        if self.return_empty_displays {
            return Ok(vec![]);
        }
        
        let native_display = NativeDisplay {
            id: "mock_native_1".to_string(),
            name: "Mock Display".to_string(),
            width: 1920,
            height: 1080,
            position_x: 0,
            position_y: 0,
            refresh_rate: Some(60.0),
            is_primary: true,
            hdr_supported: false,
            scaling_factor: 1.0,
            is_internal: true,
        };

        // Convert NativeDisplay to DisplayInfo via Adapter
        Ok(vec![native_display.into()])
    }

    fn discover_capabilities(&self, _display: &DisplayInfo) -> Result<crate::display::domain::DisplayCapabilities, PlatformError> {
        Err(PlatformError::NotImplemented("Mock platform placeholder".into()))
    }

    fn set_brightness(&self, _display: &DisplayInfo, _brightness_percent: u8) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented("Capability discovery not implemented in mock".into()))
    }

    fn get_config_path(&self) -> Result<String, PlatformError> {
        Ok("/mock/config/path".into())
    }

    fn send_notification(&self) -> Result<(), PlatformError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_platform_displays() {
        let mock = MockPlatform::new();
        let displays = mock.discover_displays().unwrap();
        assert_eq!(displays.len(), 1);
        assert_eq!(displays[0].id, "mock_native_1");
    }

    #[test]
    fn test_mock_platform_empty_displays() {
        let mut mock = MockPlatform::new();
        mock.return_empty_displays = true;
        let displays = mock.discover_displays().unwrap();
        assert_eq!(displays.len(), 0);
    }
}



