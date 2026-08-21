pub mod default;
pub mod mock;

use crate::brightness::manager::BrightnessManager;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use crate::transition::error::TransitionError;
use crate::transition::interpolator::TransitionStep;
use std::sync::Arc;

pub trait TransitionProvider: Send + Sync {
    /// Executes the transition steps asynchronously.
    fn execute_transition(
        &self,
        steps: Vec<TransitionStep>,
        brightness_manager: Arc<BrightnessManager>,
        display: DisplayInfo,
        capabilities: DisplayCapabilities,
    ) -> Result<(), TransitionError>;
}
