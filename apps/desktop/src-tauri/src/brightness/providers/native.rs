use crate::brightness::error::BrightnessError;
use crate::brightness::providers::BrightnessProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use crate::platform::hardware::wmi::manager::WmiBrightnessManager;

pub struct NativeBrightnessProvider {
    wmi_manager: WmiBrightnessManager,
}

impl NativeBrightnessProvider {
    pub fn new() -> Self {
        Self {
            wmi_manager: WmiBrightnessManager::new(),
        }
    }
}

impl BrightnessProvider for NativeBrightnessProvider {
    fn set_brightness(
        &self,
        display: &DisplayInfo,
        _capabilities: &DisplayCapabilities,
        level: u8,
    ) -> Result<(), BrightnessError> {
        if !display.is_primary {
            return Ok(());
        }
        
        // 1. Try Native WMI first (Internal laptops)
        let wmi_res = self.wmi_manager.set_brightness(level);
        
        // 2. Try Native DDC/CI (External desktop monitors)
        let ddc_success;
        
        struct EnumState {
            level: u8,
            success: bool,
        }
        
        let mut enum_state = EnumState {
            level,
            success: false,
        };

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
                let state = &mut *(lparam.0 as *mut EnumState);
                let mut count: u32 = 0;
                if GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut count).is_ok() && count > 0 {
                    let mut physical_monitors: Vec<PHYSICAL_MONITOR> = vec![PHYSICAL_MONITOR::default(); count as usize];
                    if GetPhysicalMonitorsFromHMONITOR(hmonitor, &mut physical_monitors).is_ok() {
                        for pm in &physical_monitors {
                            if SetMonitorBrightness(pm.hPhysicalMonitor, state.level as u32) != 0 {
                                state.success = true;
                            }
                        }
                        let _ = windows::Win32::Devices::Display::DestroyPhysicalMonitors(&physical_monitors);
                    }
                }
                TRUE // Continue enumeration
            }

            let _ = EnumDisplayMonitors(
                None,
                None,
                Some(monitor_enum_proc),
                LPARAM(&mut enum_state as *mut _ as isize),
            );
            ddc_success = enum_state.success;
        }
        
        if let Err(e) = wmi_res {
            if !ddc_success {
                // Both hardware attempts failed
                return Err(BrightnessError::PlatformFailure(format!("DDC/CI failed and WMI failed: {}", e)));
            }
        }
        
        Ok(())
    }

    fn get_brightness(&self, display: &DisplayInfo) -> Result<u8, BrightnessError> {
        if !display.is_primary {
            return Ok(50);
        }
        
        // For reading, WMI is the most reliable native source.
        self.wmi_manager.get_brightness()
            .map_err(|e| BrightnessError::PlatformFailure(e.to_string()))
    }
}
