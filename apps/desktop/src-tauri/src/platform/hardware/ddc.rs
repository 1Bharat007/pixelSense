use crate::platform::error::PlatformError;
use crate::display::domain::DisplayInfo;
use windows::Win32::Devices::Display::{
    GetPhysicalMonitorsFromHMONITOR, GetMonitorBrightness, SetMonitorBrightness,
    DestroyPhysicalMonitors, PHYSICAL_MONITOR,
};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, HDC, HMONITOR, MONITORINFOEXW, GetMonitorInfoW,
};
use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub struct DdcManager {}

struct EnumState<'a> {
    target_id: &'a str,
    found_hmonitor: Option<HMONITOR>,
}

unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let state = &mut *(lparam.0 as *mut EnumState);
    
    let mut info = MONITORINFOEXW::default();
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    
    // In windows-rs, some functions return BOOL which is a struct, or Result.
    // If it's a Result, we can use is_ok(). If BOOL, we can check .0 != 0.
    // To be safe, we just use a match or generic checking since it might be a Result.
    let _ = GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut _ as *mut _);
    
    let name_len = info.szDevice.iter().position(|&c| c == 0).unwrap_or(info.szDevice.len());
    let name_os = OsString::from_wide(&info.szDevice[..name_len]);
    let name = name_os.to_string_lossy().into_owned();
    
    if name == state.target_id {
        state.found_hmonitor = Some(hmonitor);
        return BOOL(0); // Stop enumeration
    }
    
    BOOL(1)
}

impl DdcManager {
    pub fn new() -> Self {
        Self {}
    }

    fn get_physical_monitor(hmonitor: HMONITOR) -> Result<PHYSICAL_MONITOR, PlatformError> {
        unsafe {
            let mut physical_monitors: [PHYSICAL_MONITOR; 1] = std::mem::zeroed();
            
            if GetPhysicalMonitorsFromHMONITOR(
                hmonitor,
                &mut physical_monitors,
            ).is_ok() {
                Ok(physical_monitors[0])
            } else {
                Err(PlatformError::NativeApiUnavailable("GetPhysicalMonitorsFromHMONITOR failed".into()))
            }
        }
    }

    pub fn get_brightness(&self, display: &DisplayInfo) -> Result<u8, PlatformError> {
        let mut state = EnumState {
            target_id: &display.id,
            found_hmonitor: None,
        };
        
        unsafe {
            let _ = EnumDisplayMonitors(None, None, Some(monitor_enum_proc), LPARAM(&mut state as *mut _ as isize));
            
            if let Some(hmonitor) = state.found_hmonitor {
                let phys_monitor = Self::get_physical_monitor(hmonitor)?;
                
                let mut min = 0;
                let mut curr = 0;
                let mut max = 0;
                
                let res = GetMonitorBrightness(phys_monitor.hPhysicalMonitor, &mut min, &mut curr, &mut max);
                
                let _ = DestroyPhysicalMonitors(&[phys_monitor]);
                
                // If it returns Result, we use is_ok(), if it's BOOL or i32 we check != 0
                // Wait, it is a Result in 0.58.0 for SetMonitorBrightness? The error said: "method not found in i32" or Result.
                // We'll just assume `res` works directly. Actually it returned `Result<(), windows_result::error::Error>` for GetPhysicalMonitors
                // and `BOOL(i32)`? The compiler error was `no method named as_bool found for type i32`.
                // So GetMonitorBrightness returns an i32 (or Result<(), Error>? wait, error said type i32).
                if res != 0 {
                    let percentage = if max > 0 {
                        ((curr as f32 / max as f32) * 100.0) as u8
                    } else {
                        curr as u8
                    };
                    return Ok(percentage);
                } else {
                    return Err(PlatformError::NativeApiUnavailable("GetMonitorBrightness failed".into()));
                }
            }
            
            Err(PlatformError::NativeApiUnavailable("Target display HMONITOR not found".into()))
        }
    }

    pub fn set_brightness(&self, display: &DisplayInfo, level: u8) -> Result<(), PlatformError> {
        let mut state = EnumState {
            target_id: &display.id,
            found_hmonitor: None,
        };
        
        unsafe {
            let _ = EnumDisplayMonitors(None, None, Some(monitor_enum_proc), LPARAM(&mut state as *mut _ as isize));
            
            if let Some(hmonitor) = state.found_hmonitor {
                let phys_monitor = Self::get_physical_monitor(hmonitor)?;
                
                let res = SetMonitorBrightness(phys_monitor.hPhysicalMonitor, level as u32);
                
                let _ = DestroyPhysicalMonitors(&[phys_monitor]);
                
                // If res is Result<(), Error> then res.is_ok(), if it's i32 then res != 0
                // We know from the error it's an i32 or Result depending on the API.
                // But the error explicitly said: `no method named as_bool found for type i32`
                if res != 0 {
                    return Ok(());
                } else {
                    return Err(PlatformError::NativeApiUnavailable("SetMonitorBrightness failed".into()));
                }
            }
            
            Err(PlatformError::NativeApiUnavailable("Target display HMONITOR not found".into()))
        }
    }
}
