use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect, GetDesktopWindow};
use windows::Win32::Graphics::Gdi::{MonitorFromWindow, GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO};
use std::mem;

pub trait ActiveWindowAnalyzer: Send + Sync {
    /// Returns true if the currently focused window is running in fullscreen mode
    /// (e.g. game, full screen video player).
    fn is_fullscreen_active(&self) -> bool;
}

pub struct WindowsWindowAnalyzer;

impl ActiveWindowAnalyzer for WindowsWindowAnalyzer {
    fn is_fullscreen_active(&self) -> bool {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.0.is_null() || hwnd == GetDesktopWindow() {
                return false;
            }

            let mut window_rect = RECT::default();
            if GetWindowRect(hwnd, &mut window_rect).is_err() {
                return false;
            }

            let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            if hmonitor.is_invalid() {
                return false;
            }

            let mut monitor_info = MONITORINFO {
                cbSize: mem::size_of::<MONITORINFO>() as u32,
                rcMonitor: RECT::default(),
                rcWork: RECT::default(),
                dwFlags: 0,
            };

            if GetMonitorInfoW(hmonitor, &mut monitor_info).as_bool() {
                // If the window completely covers the monitor rect, it is fullscreen
                if window_rect.left <= monitor_info.rcMonitor.left
                    && window_rect.top <= monitor_info.rcMonitor.top
                    && window_rect.right >= monitor_info.rcMonitor.right
                    && window_rect.bottom >= monitor_info.rcMonitor.bottom
                {
                    return true;
                }
            }

            false
        }
    }
}
