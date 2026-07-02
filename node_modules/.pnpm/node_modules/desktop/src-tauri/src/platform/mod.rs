pub mod adapter;
pub mod error;
pub mod factory;
pub mod linux;
pub mod macos;
pub mod mock;
pub mod models;
pub mod windows;
pub mod capabilities;
pub mod facade;
pub mod registry;
pub mod cache;
pub mod hardware;

use self::error::PlatformError;
use self::capabilities::PlatformCapabilities;
use crate::display::domain::DisplayInfo;

/// The central Platform interface.
/// 
/// Note: In future versions, this "god interface" will be broken down into smaller, 
/// specialized platform services such as DisplayPlatform, BrightnessPlatform, 
/// ConfigPlatform, and NotificationPlatform.
pub trait Platform: Send + Sync {
    /// Returns the active capabilities of the host OS and Hardware.
    fn get_capabilities(&self) -> Result<PlatformCapabilities, PlatformError>;

    /// Discovers connected displays.
    /// Note: May support filtering in future versions, but currently returns all displays.
    fn discover_displays(&self) -> Result<Vec<DisplayInfo>, PlatformError>;

    /// Discovers capabilities for a specific display.
    /// Note: (&self, display_id: &str) -> Result<crate::display::domain::DisplayCapabilities, PlatformError>`
    fn discover_capabilities(&self, display: &crate::display::domain::DisplayInfo) -> Result<crate::display::domain::DisplayCapabilities, PlatformError>;
    fn set_brightness(&self, display: &crate::display::domain::DisplayInfo, brightness_percent: u8) -> Result<(), PlatformError>;

    /// Sets the brightness level for a display.
    /// Future: `fn set_brightness(&self, display_id: &str, level: u32) -> Result<(), PlatformError>`
    fn set_brightness(&self) -> Result<(), PlatformError>;

    /// Retrieves the platform-specific configuration path.
    /// Future: `fn get_config_path(&self) -> Result<String, PlatformError>`
    fn get_config_path(&self) -> Result<String, PlatformError>;

    /// Sends a system notification.
    /// Future: `fn send_notification(&self, message: &str) -> Result<(), PlatformError>`
    fn send_notification(&self) -> Result<(), PlatformError>;
}

#[cfg(test)]
mod tests {
    use super::factory::create_platform;
    use super::error::PlatformError;

    #[test]
    fn test_platform_factory() {
        let platform = create_platform();
        let path = platform.get_config_path();
        
        match path {
            Ok(p) => assert_eq!(p, "/mock/config/path"),
            Err(e) => {
                match e {
                    PlatformError::NotImplemented(_) => (),
                    _ => panic!("Unexpected error: {:?}", e),
                }
            }
        }
    }
}


