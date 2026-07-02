use crate::capabilities::manager::CapabilityManager;
use crate::capabilities::providers::CapabilityProvider;

#[cfg(target_os = "windows")]
use crate::capabilities::providers::windows::WindowsCapabilityProvider;

#[cfg(target_os = "linux")]
use crate::capabilities::providers::linux::LinuxCapabilityProvider;

#[cfg(target_os = "macos")]
use crate::capabilities::providers::macos::MacOSCapabilityProvider;

#[cfg(test)]
use crate::capabilities::providers::mock::MockCapabilityProvider;

pub fn create_capability_manager() -> CapabilityManager {
    CapabilityManager::new(create_provider())
}

fn create_provider() -> Box<dyn CapabilityProvider> {
    #[cfg(test)]
    {
        Box::new(MockCapabilityProvider::new())
    }
    #[cfg(all(not(test), target_os = "windows"))]
    {
        Box::new(WindowsCapabilityProvider::new())
    }
    #[cfg(all(not(test), target_os = "linux"))]
    {
        Box::new(LinuxCapabilityProvider::new())
    }
    #[cfg(all(not(test), target_os = "macos"))]
    {
        Box::new(MacOSCapabilityProvider::new())
    }
    #[cfg(all(not(test), not(any(target_os = "windows", target_os = "linux", target_os = "macos"))))]
    {
        unimplemented!("Unsupported platform");
    }
}
