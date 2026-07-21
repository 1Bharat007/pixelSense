use crate::performance::models::PowerState;
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

pub trait PowerStateAnalyzer: Send + Sync {
    fn current_power_state(&self) -> PowerState;
}

pub struct WindowsPowerAnalyzer;

impl PowerStateAnalyzer for WindowsPowerAnalyzer {
    fn current_power_state(&self) -> PowerState {
        let mut status = SYSTEM_POWER_STATUS::default();
        unsafe {
            if GetSystemPowerStatus(&mut status).is_ok() {
                // ACLineStatus: 0 = Offline (Battery), 1 = Online (AC), 255 = Unknown
                if status.ACLineStatus == 0 {
                    return PowerState::BatteryHigh;
                }
            }
        }
        PowerState::AC // Default to AC if unknown or error
    }
}
