use std::sync::{Arc, Mutex};
use crate::brightness::manager::BrightnessManager;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use crate::transition::error::TransitionError;
use crate::transition::interpolator::TransitionStep;
use crate::transition::providers::TransitionProvider;

#[derive(Debug, Clone, PartialEq)]
pub struct MockExecutionRecord {
    pub brightness: u8,
    pub simulated_timestamp_ms: u64,
}

#[derive(Clone)]
pub struct MockTransitionProvider {
    pub records: Arc<Mutex<Vec<MockExecutionRecord>>>,
    pub interrupted: Arc<Mutex<bool>>,
}

impl MockTransitionProvider {
    pub fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(Vec::new())),
            interrupted: Arc::new(Mutex::new(false)),
        }
    }
}

impl Default for MockTransitionProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TransitionProvider for MockTransitionProvider {
    fn execute_transition(
        &self,
        steps: Vec<TransitionStep>,
        brightness_manager: Arc<BrightnessManager>,
        display: DisplayInfo,
        capabilities: DisplayCapabilities,
    ) -> Result<(), TransitionError> {
        let mut current_time = 0;
        let mut records = self.records.lock().unwrap();
        let interrupted = self.interrupted.lock().unwrap();

        for step in steps {
            if *interrupted {
                // Placeholder interrupted transition test scenario
                break;
            }

            // Execute brightness update immediately in the mock
            let _ = brightness_manager.set_brightness(&display, &capabilities, step.brightness as i32);
            
            current_time += step.delay_ms;
            records.push(MockExecutionRecord {
                brightness: step.brightness,
                simulated_timestamp_ms: current_time,
            });
        }

        Ok(())
    }
}
