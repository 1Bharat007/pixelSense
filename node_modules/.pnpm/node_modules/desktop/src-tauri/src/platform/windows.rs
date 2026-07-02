use super::{Platform, PlatformError};
use crate::display::domain::DisplayInfo;
use super::models::NativeDisplay;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW, MONITORINFOF_PRIMARY
};
use windows::Win32::Foundation::{BOOL, LPARAM, RECT};

pub struct WindowsPlatform;

impl WindowsPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowsPlatform {
    fn default() -> Self {
        Self::new()
    }
}

unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let displays = &mut *(lparam.0 as *mut Vec<NativeDisplay>);
    
    let mut info: MONITORINFOEXW = std::mem::zeroed();
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    
    if GetMonitorInfoW(hmonitor, &mut info as *mut _ as *mut _).as_bool() {
        let name_len = info.szDevice.iter().take_while(|&&c| c != 0).count();
        let name_os = OsString::from_wide(&info.szDevice[..name_len]);
        let name = name_os.to_string_lossy().into_owned();
        
        let is_primary = (info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY) != 0;
        let width = (info.monitorInfo.rcMonitor.right - info.monitorInfo.rcMonitor.left) as u32;
        let height = (info.monitorInfo.rcMonitor.bottom - info.monitorInfo.rcMonitor.top) as u32;
        let position_x = info.monitorInfo.rcMonitor.left;
        let position_y = info.monitorInfo.rcMonitor.top;

        displays.push(NativeDisplay {
            id: name.clone(), // Using device string as ID
            name,
            width,
            height,
            position_x,
            position_y,
            refresh_rate: None,
            is_primary,
        });
    }
    BOOL(1) // Continue enumeration
}

impl Platform for WindowsPlatform {
    fn discover_displays(&self) -> Result<Vec<DisplayInfo>, PlatformError> {
        let mut native_displays: Vec<NativeDisplay> = Vec::new();
        
        let lparam = LPARAM(&mut native_displays as *mut _ as isize);
        
        unsafe {
            let result = EnumDisplayMonitors(None, None, Some(monitor_enum_proc), lparam);
            if !result.as_bool() {
                return Err(PlatformError::NativeApiUnavailable("EnumDisplayMonitors failed".into()));
            }
        }
        
        let domain_displays = native_displays.into_iter().map(|nd| nd.into()).collect();
        Ok(domain_displays)
    }

    fn discover_capabilities(&self, _display: &DisplayInfo) -> Result<crate::display::domain::DisplayCapabilities, PlatformError> {
        Err(PlatformError::NotImplemented("Windows capability placeholder".into()))
    }

    fn set_brightness(&self, display: &DisplayInfo, brightness_percent: u8) -> Result<(), PlatformError> {
        // Ensure we only set brightness for internal displays as required.
        let name = display.name.to_lowercase();
        if !name.contains("internal") && !name.contains("laptop") && !display.is_primary {
            return Err(PlatformError::NotImplemented("Native brightness control only supported for internal displays".into()));
        }

        // Using PowerShell WMI as a native, dependency-free solution for internal displays.
        let script = format!("(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1, {})", brightness_percent);
        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .output()
            .map_err(|e| PlatformError::NativeApiUnavailable(e.to_string()))?;

        if !output.status.success() {
            return Err(PlatformError::NativeApiUnavailable(String::from_utf8_lossy(&output.stderr).into_owned()));
        }

        Ok(())

        Err(PlatformError::NotImplemented("Windows capability discovery not implemented".into()))
    }

    fn set_brightness(&self) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented("Windows brightness control not implemented".into()))
    }

    fn get_config_path(&self) -> Result<String, PlatformError> {
        Err(PlatformError::NotImplemented("Windows config path not implemented".into()))
    }

    fn send_notification(&self) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented("Windows notifications not implemented".into()))
    }
}



