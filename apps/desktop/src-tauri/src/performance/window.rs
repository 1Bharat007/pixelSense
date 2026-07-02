pub trait ActiveWindowAnalyzer: Send + Sync {
    /// Returns true if the currently focused window is running in fullscreen mode
    /// (e.g. game, full screen video player).
    fn is_fullscreen_active(&self) -> bool;
}

#[cfg(target_os = "windows")]
pub struct WindowsWindowAnalyzer;

#[cfg(target_os = "windows")]
impl ActiveWindowAnalyzer for WindowsWindowAnalyzer {
    fn is_fullscreen_active(&self) -> bool {
        // TODO: Implement actual user32 API calls
        // For example: GetForegroundWindow, GetWindowRect vs MonitorRect
        false
    }
}

pub struct MockWindowAnalyzer {
    is_fullscreen: std::sync::Mutex<bool>,
}

impl MockWindowAnalyzer {
    pub fn new(initial: bool) -> Self {
        Self {
            is_fullscreen: std::sync::Mutex::new(initial),
        }
    }
    
    pub fn set_fullscreen(&self, fullscreen: bool) {
        *self.is_fullscreen.lock().unwrap() = fullscreen;
    }
}

impl ActiveWindowAnalyzer for MockWindowAnalyzer {
    fn is_fullscreen_active(&self) -> bool {
        *self.is_fullscreen.lock().unwrap()
    }
}
