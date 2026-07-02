use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use crate::platform::error::PlatformError;
use crate::ambient::models::AmbientReading;
use crate::screen_analysis::frame::scaler::RawFrameBuffer;

/// PlatformFacade acts as the single boundary for all native OS calls.
/// Subsystems interact with these specialized traits, never directly with Win32/macOS/Linux APIs.

pub trait DisplayPlatform: Send + Sync {
    fn discover_displays(&self) -> Result<Vec<DisplayInfo>, PlatformError>;
    fn get_display_capabilities(&self, display: &DisplayInfo) -> Result<DisplayCapabilities, PlatformError>;
}

pub trait BrightnessPlatform: Send + Sync {
    fn set_internal_brightness(&self, level: u8) -> Result<(), PlatformError>;
    fn set_external_brightness(&self, display: &DisplayInfo, level: u8) -> Result<(), PlatformError>;
    fn read_hardware_brightness(&self, display: &DisplayInfo) -> Result<u8, PlatformError>;
}

pub trait CapturePlatform: Send + Sync {
    fn acquire_next_frame(&self, display_id: &str) -> Result<RawFrameBuffer, PlatformError>;
}

pub trait SensorPlatform: Send + Sync {
    fn read_ambient_light(&self) -> Result<AmbientReading, PlatformError>;
}

pub trait WindowPlatform: Send + Sync {
    fn get_active_window_executable(&self) -> Result<String, PlatformError>;
}

pub trait PowerPlatform: Send + Sync {
    fn is_on_battery(&self) -> Result<bool, PlatformError>;
    fn is_battery_saver_active(&self) -> Result<bool, PlatformError>;
}

pub trait SessionPlatform: Send + Sync {
    fn is_session_locked(&self) -> Result<bool, PlatformError>;
}

/// The overarching PlatformFacade aggregates these specialized platforms.
pub trait PlatformFacade: Send + Sync {
    fn display(&self) -> &dyn DisplayPlatform;
    fn brightness(&self) -> &dyn BrightnessPlatform;
    fn capture(&self) -> &dyn CapturePlatform;
    fn sensor(&self) -> &dyn SensorPlatform;
    fn window(&self) -> &dyn WindowPlatform;
    fn power(&self) -> &dyn PowerPlatform;
    fn session(&self) -> &dyn SessionPlatform;
}
