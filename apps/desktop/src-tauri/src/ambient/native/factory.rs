use crate::ambient::provider::AmbientProvider;
use std::sync::Arc;

#[cfg(target_os = "windows")]
pub fn create_native_provider() -> Arc<dyn AmbientProvider> {
    use crate::ambient::native::windows::WindowsAmbientProvider;
    Arc::new(WindowsAmbientProvider::new())
}

#[cfg(target_os = "linux")]
pub fn create_native_provider() -> Arc<dyn AmbientProvider> {
    use crate::ambient::native::linux::LinuxAmbientProvider;
    Arc::new(LinuxAmbientProvider)
}

#[cfg(target_os = "macos")]
pub fn create_native_provider() -> Arc<dyn AmbientProvider> {
    use crate::ambient::native::macos::MacosAmbientProvider;
    Arc::new(MacosAmbientProvider)
}

// Fallback for tests or unknown OS
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn create_native_provider() -> Arc<dyn AmbientProvider> {
    use crate::ambient::native::mock::MockAmbientProvider;
    Arc::new(MockAmbientProvider::new())
}
