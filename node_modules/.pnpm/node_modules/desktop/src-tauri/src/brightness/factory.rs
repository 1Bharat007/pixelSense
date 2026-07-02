use crate::brightness::manager::BrightnessManager;
use crate::brightness::providers::BrightnessProvider;

#[cfg(target_os = "windows")]
use crate::brightness::providers::windows::WindowsBrightnessProvider;

#[cfg(target_os = "linux")]
use crate::brightness::providers::linux::LinuxBrightnessProvider;

#[cfg(target_os = "macos")]
use crate::brightness::providers::macos::MacOSBrightnessProvider;

#[cfg(test)]
use crate::brightness::providers::mock::MockBrightnessProvider;

pub fn create_brightness_manager() -> BrightnessManager {
    BrightnessManager::new(create_provider())
}

fn create_provider() -> Box<dyn BrightnessProvider> {
    #[cfg(test)]
    {
        Box::new(MockBrightnessProvider::new())
    }
    #[cfg(all(not(test), target_os = "windows"))]
    {
        Box::new(WindowsBrightnessProvider::new())
    }
    #[cfg(all(not(test), target_os = "linux"))]
    {
        Box::new(LinuxBrightnessProvider::new())
    }
    #[cfg(all(not(test), target_os = "macos"))]
    {
        Box::new(MacOSBrightnessProvider::new())
    }
    #[cfg(all(not(test), not(any(target_os = "windows", target_os = "linux", target_os = "macos"))))]
    {
        unimplemented!("Unsupported platform");
    }
}
