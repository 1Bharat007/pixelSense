use crate::brightness::error::BrightnessError;
use crate::decision::error::DecisionError;
use crate::transition::error::TransitionError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdaptiveError {
    DecisionFailed(String),
    TransitionFailed(String),
    BrightnessFailed(String),
    ConfidenceTooLow(f32),
    AdaptiveDisabled,
    PipelineInterrupted,
}

impl From<DecisionError> for AdaptiveError {
    fn from(e: DecisionError) -> Self {
        Self::DecisionFailed(e.to_string())
    }
}

impl From<TransitionError> for AdaptiveError {
    fn from(e: TransitionError) -> Self {
        Self::TransitionFailed(e.to_string())
    }
}

impl From<BrightnessError> for AdaptiveError {
    fn from(e: BrightnessError) -> Self {
        Self::BrightnessFailed(e.to_string())
    }
}

impl std::fmt::Display for AdaptiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdaptiveError::DecisionFailed(msg) => write!(f, "Decision failed: {}", msg),
            AdaptiveError::TransitionFailed(msg) => write!(f, "Transition failed: {}", msg),
            AdaptiveError::BrightnessFailed(msg) => write!(f, "Brightness failed: {}", msg),
            AdaptiveError::ConfidenceTooLow(c) => write!(f, "Confidence too low: {}", c),
            AdaptiveError::AdaptiveDisabled => write!(f, "Adaptive brightness is disabled"),
            AdaptiveError::PipelineInterrupted => write!(f, "Pipeline interrupted"),
        }
    }
}

impl std::error::Error for AdaptiveError {}
