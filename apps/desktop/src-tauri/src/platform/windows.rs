use super::{Platform, PlatformError};
use crate::display::domain::DisplayInfo;
use super::models::NativeDisplay;
use crate::platform::capabilities::PlatformCapabilities;
use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::System::ProcessStatus::GetProcessImageFileNameW;
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW, EnumDisplaySettingsW, DEVMODEW, ENUM_CURRENT_SETTINGS
};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, IDXGIFactory1, IDXGIOutput6
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020;
use windows::core::Interface;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use crate::platform::hardware::wmi::manager::WmiBrightnessManager;
use crate::platform::hardware::ddc::DdcManager;
use crate::platform::hardware::dxgi::manager::DxgiDeviceManager;
use crate::platform::hardware::sensor::manager::SensorSession;

const MONITORINFOF_PRIMARY: u32 = 1;

pub struct WindowsPlatform {
    #[allow(dead_code)] // Reserved for future capability checking
    capabilities: PlatformCapabilities,
    wmi_brightness: WmiBrightnessManager,
    ddc_brightness: DdcManager,
    #[allow(dead_code)] // Reserved for future DXGI implementation
    dxgi_manager: DxgiDeviceManager,
    sensor_session: SensorSession,
}

impl WindowsPlatform {
    pub fn new() -> Self {
        Self {
            capabilities: PlatformCapabilities::detect(),
            wmi_brightness: WmiBrightnessManager::new(),
            ddc_brightness: DdcManager::new(),
            dxgi_manager: DxgiDeviceManager::new(),
            sensor_session: SensorSession::new(),
        }
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

        // Get Refresh Rate
        let mut dev_mode: DEVMODEW = std::mem::zeroed();
        dev_mode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;
        let mut refresh_rate = None;
        use windows::core::PCWSTR;
        if EnumDisplaySettingsW(
            PCWSTR(info.szDevice.as_ptr()),
            ENUM_CURRENT_SETTINGS,
            &mut dev_mode as *mut _
        ).as_bool() {
            if dev_mode.dmDisplayFrequency > 1 {
                refresh_rate = Some(dev_mode.dmDisplayFrequency as f32);
            }
        }

        // Get Scaling
        let mut dpi_x = 96;
        let mut dpi_y = 96;
        let _ = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);
        let scaling_factor = dpi_x as f32 / 96.0;

        // Get HDR
        let mut hdr_supported = false;
        if let Ok(factory) = CreateDXGIFactory1::<IDXGIFactory1>() {
            let mut i = 0;
            while let Ok(adapter) = factory.EnumAdapters1(i) {
                let mut j = 0;
                while let Ok(output) = adapter.EnumOutputs(j) {
                    if let Ok(desc) = output.GetDesc() {
                        if desc.Monitor == hmonitor {
                            if let Ok(output6) = output.cast::<IDXGIOutput6>() {
                                if let Ok(desc1) = output6.GetDesc1() {
                                    if desc1.ColorSpace == DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020 {
                                        hdr_supported = true;
                                    }
                                }
                            }
                        }
                    }
                    j += 1;
                }
                i += 1;
            }
        }

        // For now, heuristic for internal: primary monitor on a system that has a battery.
        // Or we just default to true for primary display. We'll rely on WMI vs DDC to route correctly.
        let is_internal = is_primary; // A reasonable approximation for laptops without full CCD implementation

        displays.push(NativeDisplay {
            id: name.clone(), // Using device string as ID
            name,
            width,
            height,
            position_x,
            position_y,
            refresh_rate,
            is_primary,
            hdr_supported,
            scaling_factor,
            is_internal,
        });
    }
    BOOL(1) // Continue enumeration
}

use crate::platform::facade::{DisplayPlatform, BrightnessPlatform, CapturePlatform, SensorPlatform, WindowPlatform, PowerPlatform, SessionPlatform, PlatformFacade};

impl DisplayPlatform for WindowsPlatform {
    fn discover_displays(&self) -> Result<Vec<DisplayInfo>, PlatformError> {
        let mut native_displays: Vec<NativeDisplay> = Vec::new();
        let lparam = LPARAM(&mut native_displays as *mut _ as isize);
        
        unsafe {
            let result = EnumDisplayMonitors(None, None, Some(monitor_enum_proc), lparam);
            if !result.as_bool() {
                return Err(PlatformError::NativeApiUnavailable("EnumDisplayMonitors failed".into()));
            }
        }
        
        Ok(native_displays.into_iter().map(|nd| nd.into()).collect())
    }

    fn get_display_capabilities(&self, _display: &DisplayInfo) -> Result<crate::display::domain::DisplayCapabilities, PlatformError> {
        Ok(crate::display::domain::DisplayCapabilities {
            brightness: true,
            hdr: false,
            ddc_ci: true, // Will be dynamically queried in future PRs
        })
    }
}

impl BrightnessPlatform for WindowsPlatform {
    fn set_internal_brightness(&self, level: u8) -> Result<(), PlatformError> {
        self.wmi_brightness.set_brightness(level)
    }

    fn set_external_brightness(&self, display: &DisplayInfo, level: u8) -> Result<(), PlatformError> {
        self.ddc_brightness.set_brightness(display, level)
    }

    fn read_hardware_brightness(&self, display: &DisplayInfo) -> Result<u8, PlatformError> {
        let name = display.name.to_lowercase();
        let is_internal = name.contains("internal") || name.contains("laptop") || display.is_primary;
        if is_internal {
            self.wmi_brightness.get_brightness()
        } else {
            self.ddc_brightness.get_brightness(display)
        }
    }
}

impl CapturePlatform for WindowsPlatform {
    fn acquire_next_frame(&self, _display_id: &str) -> Result<crate::screen_analysis::frame::scaler::RawFrameBuffer, PlatformError> {
        Ok(crate::screen_analysis::frame::scaler::RawFrameBuffer::new(vec![0, 0, 0, 255], 1, 1))
    }
}

impl SensorPlatform for WindowsPlatform {
    fn read_ambient_light(&self) -> Result<crate::ambient::models::AmbientReading, PlatformError> {
        let lux = self.sensor_session.read_lux()?;
        
        use crate::ambient::models::{AmbientReading, AmbientSensorType, AmbientEnvironment, AmbientQuality};
        use crate::background::models::now_ms;

        Ok(AmbientReading {
            source_id: "windows_native_als".into(),
            sensor_name: "Windows Native ALS (Facade)".into(),
            lux,
            normalized_lux: 0.0,
            environment: AmbientEnvironment::Office,
            confidence: 1.0,
            sensor_type: AmbientSensorType::NativeSensor,
            timestamp: now_ms(),
            quality: AmbientQuality::Good,
            is_stable: true,
            reading_duration_ms: 0,
            is_estimated: false,
        })
    }
}

impl WindowPlatform for WindowsPlatform {
    fn get_active_window_executable(&self) -> Result<String, PlatformError> {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.0 == std::ptr::null_mut() {
                return Err(PlatformError::NativeApiUnavailable("GetForegroundWindow failed".into()));
            }

            let mut process_id = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut process_id));

            let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id)
                .map_err(|e| PlatformError::NativeApiUnavailable(format!("OpenProcess failed: {}", e)))?;

            let mut buffer = [0u16; 512];
            let len = GetProcessImageFileNameW(process, &mut buffer);
            if len == 0 {
                return Err(PlatformError::NativeApiUnavailable("GetProcessImageFileNameW failed".into()));
            }

            let path = std::ffi::OsString::from_wide(&buffer[..len as usize]);
            let path_str = path.to_string_lossy().to_string();
            // Extract just the executable name
            let exe_name = std::path::Path::new(&path_str)
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or(path_str);

            Ok(exe_name)
        }
    }
}

impl PowerPlatform for WindowsPlatform {
    fn is_on_battery(&self) -> Result<bool, PlatformError> {
        let mut status = SYSTEM_POWER_STATUS::default();
        unsafe {
            if GetSystemPowerStatus(&mut status).is_err() {
                return Err(PlatformError::NativeApiUnavailable("GetSystemPowerStatus failed".into()));
            }
        }
        // ACLineStatus: 0 = Offline (battery), 1 = Online (AC)
        Ok(status.ACLineStatus == 0)
    }
    
    fn is_battery_saver_active(&self) -> Result<bool, PlatformError> {
        let mut status = SYSTEM_POWER_STATUS::default();
        unsafe {
            if GetSystemPowerStatus(&mut status).is_err() {
                return Err(PlatformError::NativeApiUnavailable("GetSystemPowerStatus failed".into()));
            }
        }
        // SystemStatusFlag: 1 = Battery saver is ON.
        Ok(status.SystemStatusFlag == 1)
    }
}

impl SessionPlatform for WindowsPlatform {
    fn is_session_locked(&self) -> Result<bool, PlatformError> {
        Ok(false)
    }
}

impl PlatformFacade for WindowsPlatform {
    fn display(&self) -> &dyn DisplayPlatform { self }
    fn brightness(&self) -> &dyn BrightnessPlatform { self }
    fn capture(&self) -> &dyn CapturePlatform { self }
    fn sensor(&self) -> &dyn SensorPlatform { self }
    fn window(&self) -> &dyn WindowPlatform { self }
    fn power(&self) -> &dyn PowerPlatform { self }
    fn session(&self) -> &dyn SessionPlatform { self }
}

impl Platform for WindowsPlatform {
    fn get_capabilities(&self) -> Result<PlatformCapabilities, PlatformError> {
        let mut caps = PlatformCapabilities::default();
        caps.ambient_sensor = true;
        caps.desktop_duplication = true;
        caps.ddc_ci = true;
        caps.internal_monitor_brightness = true;
        caps.hdr = true;
        caps.night_light_detection = true;
        caps.refresh_rate_query = true;
        caps.power_state = true;
        caps.window_tracking = true;
        caps.display_enumeration = true;
        Ok(caps)
    }

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
        let name = display.name.to_lowercase();
        // Determine if it's an internal display (WMI) or external (DDC)
        if name.contains("internal") || name.contains("laptop") || display.is_primary {
            self.wmi_brightness.set_brightness(brightness_percent)
        } else {
            // Placeholder for DDC integration which will come next
            Err(PlatformError::NotImplemented("DDC brightness control not implemented".into()))
        }
    }

    fn get_config_path(&self) -> Result<String, PlatformError> {
        Err(PlatformError::NotImplemented("Windows config path not implemented".into()))
    }

    fn send_notification(&self) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented("Windows notifications not implemented".into()))
    }
}



