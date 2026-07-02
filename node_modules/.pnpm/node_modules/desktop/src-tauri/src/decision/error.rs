use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DecisionError {
    InvalidInputData(String),
    CalculationFailed(String),
    StrategyUnavailable(String),
    NotImplemented(String),
}

impl std::fmt::Display for DecisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionError::InvalidInputData(msg) => write!(f, "Invalid input data: {}", msg),
            DecisionError::CalculationFailed(msg) => write!(f, "Calculation failed: {}", msg),
            DecisionError::StrategyUnavailable(msg) => write!(f, "Strategy unavailable: {}", msg),
            DecisionError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
        }
    }
}

impl std::error::Error for DecisionError {}
