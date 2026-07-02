use super::Platform;

#[cfg(target_os = "windows")]
use super::windows::WindowsPlatform;

#[cfg(target_os = "linux")]
use super::linux::LinuxPlatform;

#[cfg(target_os = "macos")]
use super::macos::MacOSPlatform;

#[cfg(test)]
use super::mock::MockPlatform;

/// Factory function to select the appropriate platform implementation.
/// In tests, this will return the MockPlatform.
pub fn create_platform() -> Box<dyn Platform> {
    #[cfg(test)]
    {
        Box::new(MockPlatform::new())
    }
    #[cfg(all(not(test), target_os = "windows"))]
    {
        Box::new(WindowsPlatform::new())
    }
    #[cfg(all(not(test), target_os = "linux"))]
    {
        Box::new(LinuxPlatform::new())
    }
    #[cfg(all(not(test), target_os = "macos"))]
    {
        Box::new(MacOSPlatform::new())
    }
    #[cfg(all(not(test), not(any(target_os = "windows", target_os = "linux", target_os = "macos"))))]
    {
        unimplemented!("Unsupported platform");
    }
}
