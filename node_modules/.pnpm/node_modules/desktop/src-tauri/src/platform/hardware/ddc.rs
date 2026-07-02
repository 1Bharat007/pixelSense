use crate::platform::error::PlatformError;

/// DDCTransport defines the physical layer transport (e.g. I2C) for DDC commands.
pub trait DDCTransport: Send + Sync {
    fn write_command(&self, command: &[u8]) -> Result<(), PlatformError>;
    fn read_reply(&self, buffer: &mut [u8]) -> Result<usize, PlatformError>;
}

/// DDCController handles formatting commands and parsing replies per the VESA DDC/CI spec.
pub struct DDCController {
    transport: Box<dyn DDCTransport>,
}

impl DDCController {
    pub fn new(transport: Box<dyn DDCTransport>) -> Self {
        Self { transport }
    }

    pub fn set_vcp_feature(&self, vcp_code: u8, value: u16) -> Result<(), PlatformError> {
        // Construct the VESA SET_VCP_FEATURE packet
        let _command = vec![
            0x51,             // Source ID
            0x84,             // Length
            0x03,             // SET_VCP_FEATURE opcode
            vcp_code,         // Target VCP code
            (value >> 8) as u8, // High byte
            (value & 0xFF) as u8, // Low byte
            // Checksum would be appended here
        ];
        
        // This is a stub for the architecture. Actual I2C writes will be done here.
        Err(PlatformError::NotImplemented("DDC VCP Set not yet fully wired to I2C".into()))
    }
}

use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Devices::Display::{GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, DestroyPhysicalMonitor, PHYSICAL_MONITOR};
use std::sync::Mutex;
use std::collections::HashMap;

/// Manages the lifecycle of Physical Monitor handles to prevent leaks and reconnect delays.
pub struct PhysicalMonitorPool {
    monitors: Mutex<HashMap<isize, Vec<PHYSICAL_MONITOR>>>,
}

impl PhysicalMonitorPool {
    pub fn new() -> Self {
        Self {
            monitors: Mutex::new(HashMap::new()),
        }
    }

    /// Acquires the physical monitors for a given HMONITOR.
    /// Reuses cached handles if available.
    pub fn get_monitors(&self, hmonitor: HMONITOR) -> Result<Vec<PHYSICAL_MONITOR>, PlatformError> {
        let mut map = self.monitors.lock().unwrap();
        
        if let Some(cached) = map.get(&hmonitor.0) {
            return Ok(cached.clone());
        }

        unsafe {
            let mut count = 0;
            if GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut count).is_err() {
                return Err(PlatformError::NativeApiUnavailable("GetNumberOfPhysicalMonitors failed".into()));
            }

            let mut physical_monitors: Vec<PHYSICAL_MONITOR> = vec![PHYSICAL_MONITOR::default(); count as usize];
            if GetPhysicalMonitorsFromHMONITOR(hmonitor, physical_monitors.as_mut_slice()).is_err() {
                return Err(PlatformError::NativeApiUnavailable("GetPhysicalMonitors failed".into()));
            }

            map.insert(hmonitor.0, physical_monitors.clone());
            Ok(physical_monitors)
        }
    }

    /// Clears the pool and securely destroys all physical monitor handles.
    pub fn flush(&self) {
        let mut map = self.monitors.lock().unwrap();
        for (_, monitors) in map.drain() {
            for mon in monitors {
                unsafe {
                    let _ = DestroyPhysicalMonitor(mon.hPhysicalMonitor);
                }
            }
        }
    }
}

impl Drop for PhysicalMonitorPool {
    fn drop(&mut self) {
        self.flush();
    }
}

