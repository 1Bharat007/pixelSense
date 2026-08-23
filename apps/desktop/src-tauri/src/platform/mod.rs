pub mod adapter;
pub mod application;
pub mod cache;
pub mod capabilities;
pub mod error;
pub mod facade;
pub mod factory;
pub mod hardware;
pub mod models;
pub mod registry;
pub mod tests;
pub mod windows;

use self::capabilities::PlatformCapabilities;
use self::error::PlatformError;
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
    fn discover_capabilities(
        &self,
        display: &crate::display::domain::DisplayInfo,
    ) -> Result<crate::display::domain::DisplayCapabilities, PlatformError>;
    /// Sets the brightness level for a display.
    /// Future: `fn set_brightness(&self, display_id: &str, level: u32) -> Result<(), PlatformError>`
    fn set_brightness(
        &self,
        display: &crate::display::domain::DisplayInfo,
        brightness_percent: u8,
    ) -> Result<(), PlatformError>;

    /// Retrieves the platform-specific configuration path.
    /// Future: `fn get_config_path(&self) -> Result<String, PlatformError>`
    fn get_config_path(&self) -> Result<String, PlatformError>;

    /// Sends a system notification.
    /// Future: `fn send_notification(&self, message: &str) -> Result<(), PlatformError>`
    fn send_notification(&self) -> Result<(), PlatformError>;
}
