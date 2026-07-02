use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransitionError {
    InvalidDuration(String),
    ExecutionFailed(String),
    TransitionAlreadyRunning(String),
    NotImplemented(String),
}

impl std::fmt::Display for TransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionError::InvalidDuration(msg) => write!(f, "Invalid duration: {}", msg),
            TransitionError::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            TransitionError::TransitionAlreadyRunning(msg) => write!(f, "Transition already running: {}", msg),
            TransitionError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
        }
    }
}

impl std::error::Error for TransitionError {}
