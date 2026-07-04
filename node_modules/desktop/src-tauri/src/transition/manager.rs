use std::sync::Arc;
use crate::brightness::manager::BrightnessManager;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use crate::transition::config::TransitionConfig;
use crate::transition::error::TransitionError;
use crate::transition::interpolator::LinearInterpolator;
use crate::transition::providers::TransitionProvider;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecutionMode {
    Immediate,
    Transition,
}

pub struct TransitionManager {
    provider: Box<dyn TransitionProvider>,
    brightness_manager: Arc<BrightnessManager>,
    config: TransitionConfig,
}

impl TransitionManager {
    pub fn new(
        provider: Box<dyn TransitionProvider>,
        brightness_manager: Arc<BrightnessManager>,
        config: TransitionConfig,
    ) -> Self {
        Self {
            provider,
            brightness_manager,
            config,
        }
    }

    pub fn transition_brightness(
        &self,
        display: &DisplayInfo,
        capabilities: &DisplayCapabilities,
        current_brightness: u8,
        target_brightness: u8,
        duration_ms: u64,
        mode: ExecutionMode,
    ) -> Result<(), TransitionError> {
        let actual_duration = match mode {
            ExecutionMode::Immediate => 0,
            ExecutionMode::Transition => duration_ms,
        };

        // Generate steps (Business Logic)
        let steps = LinearInterpolator::interpolate(
            current_brightness,
            target_brightness,
            actual_duration,
            &self.config,
        );

        // Delegate execution to the provider
        self.provider.execute_transition(
            steps,
            Arc::clone(&self.brightness_manager),
            display.clone(),
            capabilities.clone(),
        )
    }
}
