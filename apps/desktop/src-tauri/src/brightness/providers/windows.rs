use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use std::process::Command;

pub struct WindowsBrightnessProvider {}

impl WindowsBrightnessProvider {
    pub fn new() -> Self {
        Self {}
    }
    
    // Internal WMI helper
    fn set_wmi_brightness(&self, level: u8) -> Result<(), BrightnessError> {
        // We use powershell here for WMI because WmiMonitorBrightnessMethods
        // method execution via COM requires highly specific variant boxing 
        // which can be brittle across Windows versions in Rust.
        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(format!("(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1, {})", level))
            .output()
            .map_err(|e| BrightnessError::PlatformFailure(e.to_string()))?;
            
        if !output.status.success() {
            return Err(BrightnessError::PlatformFailure("WMI brightness command failed".to_string()));
        }
        
        // Verification (Read-back)
        let verify_output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg("(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightness).CurrentBrightness")
            .output()
            .map_err(|e| BrightnessError::PlatformFailure(e.to_string()))?;
            
        if let Ok(stdout) = String::from_utf8(verify_output.stdout) {
            if let Ok(current) = stdout.trim().parse::<u8>() {
                // We allow a small delta because some panels snap to nearest 5%
                if current.abs_diff(level) > 5 {
                    return Err(BrightnessError::PlatformFailure("WMI readback verification failed".to_string()));
                }
            }
        }
            
        Ok(())
    }
}

impl BrightnessProvider for WindowsBrightnessProvider {
    fn set_brightness(
        &self,
        display: &DisplayInfo,
        _capabilities: &DisplayCapabilities,
        level: u8,
    ) -> Result<(), BrightnessError> {
        // Primary display only for RC-14
        if !display.is_primary {
            return Ok(());
        }
        
        // Try WMI first (Internal laptops)
        if self.set_wmi_brightness(level).is_ok() {
            return Ok(());
        }
        
        // If WMI fails (e.g. external desktop monitor), try DDC/CI natively via windows-rs
        unsafe {
            use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR};
            use windows::Win32::Devices::Display::{GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, SetMonitorBrightness, PHYSICAL_MONITOR};
            use windows::Win32::Foundation::{BOOL, LPARAM, TRUE};

            unsafe extern "system" fn monitor_enum_proc(
                hmonitor: HMONITOR,
                _hdc: HDC,
                _lprect: *mut windows::Win32::Foundation::RECT,
                lparam: LPARAM,
            ) -> BOOL {
                let level = lparam.0 as u8;
                let mut count: u32 = 0;
                if GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut count).is_ok() && count > 0 {
                    let mut physical_monitors: Vec<PHYSICAL_MONITOR> = vec![PHYSICAL_MONITOR::default(); count as usize];
                    if GetPhysicalMonitorsFromHMONITOR(hmonitor, &mut physical_monitors).is_ok() {
                        for pm in physical_monitors {
                            let _ = SetMonitorBrightness(pm.hPhysicalMonitor, level as u32);
                        }
                    }
                }
                TRUE // Continue enumeration
            }

            let _ = EnumDisplayMonitors(
                None,
                None,
                Some(monitor_enum_proc),
                LPARAM(level as isize),
            );
        }
        
        Ok(())
    }

    fn get_brightness(&self, display: &DisplayInfo) -> Result<u8, BrightnessError> {
        if !display.is_primary {
            return Ok(50);
        }
        
        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg("(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightness -ErrorAction Stop).CurrentBrightness")
            .output()
            .map_err(|_| BrightnessError::PlatformFailure("Windows doesn't allow brightness control on this display.".to_string()))?;
            
        if output.status.success() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                if let Ok(current) = stdout.trim().parse::<u8>() {
                    return Ok(current);
                }
            }
        }
        
        Err(BrightnessError::PlatformFailure("This monitor doesn't support automatic brightness.".to_string()))
    }
}
