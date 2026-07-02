use crate::performance::models::PowerState;

pub trait PowerStateAnalyzer: Send + Sync {
    fn current_power_state(&self) -> PowerState;
}

#[cfg(target_os = "windows")]
pub struct WindowsPowerAnalyzer;

#[cfg(target_os = "windows")]
impl PowerStateAnalyzer for WindowsPowerAnalyzer {
    fn current_power_state(&self) -> PowerState {
        // TODO: Wire to actual GetSystemPowerStatus.
        // For now, return AC to simulate connected to power.
        PowerState::AC
    }
}

// Fallback/Mock analyzer
pub struct MockPowerAnalyzer {
    state: std::sync::Mutex<PowerState>,
}

impl MockPowerAnalyzer {
    pub fn new(initial: PowerState) -> Self {
        Self {
            state: std::sync::Mutex::new(initial),
        }
    }
    
    pub fn set_state(&self, state: PowerState) {
        *self.state.lock().unwrap() = state;
    }
}

impl PowerStateAnalyzer for MockPowerAnalyzer {
    fn current_power_state(&self) -> PowerState {
        *self.state.lock().unwrap()
    }
}
