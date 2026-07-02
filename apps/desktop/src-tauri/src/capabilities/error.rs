use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CapabilityError {
    EvaluationFailed(String),
    UnsupportedDisplay(String),
    PlatformFailure(String),
    CapabilityUnavailable(String),
    NotImplemented(String),
}

impl std::fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CapabilityError::EvaluationFailed(msg) => write!(f, "Evaluation failed: {}", msg),
            CapabilityError::UnsupportedDisplay(msg) => write!(f, "Unsupported display: {}", msg),
            CapabilityError::PlatformFailure(msg) => write!(f, "Platform failure: {}", msg),
            CapabilityError::CapabilityUnavailable(msg) => write!(f, "Capability unavailable: {}", msg),
            CapabilityError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
        }
    }
}

impl std::error::Error for CapabilityError {}
