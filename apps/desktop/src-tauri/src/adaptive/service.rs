use std::sync::{Arc, Mutex};
use crate::adaptive::config::AdaptiveConfig;
use crate::adaptive::error::AdaptiveError;
use crate::adaptive::state::BrightnessState;
use crate::decision::manager::DecisionManager;
use crate::decision::models::DecisionContext;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use crate::transition::manager::{ExecutionMode, TransitionManager};

pub struct AdaptiveBrightnessService {
    decision_manager: DecisionManager,
    transition_manager: TransitionManager,
    config: AdaptiveConfig,
    state: Arc<Mutex<BrightnessState>>,
}

impl AdaptiveBrightnessService {
    pub fn new(
        decision_manager: DecisionManager,
        transition_manager: TransitionManager,
        config: AdaptiveConfig,
        state: Arc<Mutex<BrightnessState>>,
    ) -> Self {
        Self {
            decision_manager,
            transition_manager,
            config,
            state,
        }
    }

    /// Orchestrates the entire adaptive brightness pipeline.
    /// 1. Verifies configuration.
    /// 2. Requests target brightness from DecisionManager.
    /// 3. Validates confidence.
    /// 4. Dispatches to TransitionManager.
    pub fn execute_pipeline(
        &self,
        display: &DisplayInfo,
        capabilities: &DisplayCapabilities,
        context: &DecisionContext,
    ) -> Result<(), AdaptiveError> {
        if !self.config.adaptive_enabled {
            return Err(AdaptiveError::AdaptiveDisabled);
        }

        // 1. Get recommendation
        let decision_result = self.decision_manager.decide_brightness(context)?;

        // 2. Enforce confidence policy
        // (Future: 0.90 -> Immediate, 0.50 -> Normal, 0.20 -> Ignore)
        if decision_result.confidence < self.config.confidence_threshold {
            return Err(AdaptiveError::ConfidenceTooLow(decision_result.confidence));
        }

        // 3. Determine execution mode
        let mode = if self.config.transition_enabled {
            ExecutionMode::Transition
        } else {
            ExecutionMode::Immediate
        };

        // 4. Get current state
        let current_brightness = {
            let state = self.state.lock().unwrap();
            state.get_brightness(&display.id)
        };

        // 5. Execute transition
        self.transition_manager.transition_brightness(
            display,
            capabilities,
            current_brightness,
            decision_result.recommended_brightness,
            self.config.transition_duration_ms,
            mode,
        )?;

        // 6. Update state
        {
            let mut state = self.state.lock().unwrap();
            state.update_brightness(&display.id, decision_result.recommended_brightness);
        }

        Ok(())
    }
}
