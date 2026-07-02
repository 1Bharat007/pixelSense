use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::brightness::manager::BrightnessManager;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use crate::transition::error::TransitionError;
use crate::transition::interpolator::TransitionStep;
use crate::transition::providers::TransitionProvider;

pub struct DefaultTransitionProvider;

impl DefaultTransitionProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultTransitionProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TransitionProvider for DefaultTransitionProvider {
    fn execute_transition(
        &self,
        steps: Vec<TransitionStep>,
        brightness_manager: Arc<BrightnessManager>,
        display: DisplayInfo,
        capabilities: DisplayCapabilities,
    ) -> Result<(), TransitionError> {
        // Lifecycle:
        // New Transition -> (TODO: Cancel Previous Transition) -> Start New Transition
        
        thread::spawn(move || {
            for step in steps {
                // TODO: Check cancellation token here

                // Execute brightness update
                let _ = brightness_manager.set_brightness(&display, &capabilities, step.brightness as i32);
                
                // Sleep for the tick interval
                if step.delay_ms > 0 {
                    thread::sleep(Duration::from_millis(step.delay_ms));
                }
            }
        });

        Ok(())
    }
}
