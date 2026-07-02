use crate::display::manager::DisplayManager;
use crate::display::providers::DisplayProvider;

#[cfg(target_os = "windows")]
use crate::display::providers::windows::WindowsProvider;

#[cfg(target_os = "linux")]
use crate::display::providers::linux::LinuxProvider;

#[cfg(target_os = "macos")]
use crate::display::providers::macos::MacOSProvider;

pub fn create_display_manager() -> DisplayManager {
    DisplayManager::new(create_provider())
}

fn create_provider() -> Box<dyn DisplayProvider> {
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsProvider::new())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxProvider::new())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(MacOSProvider::new())
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        unimplemented!("Unsupported platform");
    }
}
