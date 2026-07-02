use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BrightnessError {
    UnsupportedDisplay(String),
    PlatformFailure(String),
    BrightnessOutOfRange(String),
    InvalidValue(String),
    NotImplemented(String),
}

impl std::fmt::Display for BrightnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrightnessError::UnsupportedDisplay(msg) => write!(f, "Unsupported display: {}", msg),
            BrightnessError::PlatformFailure(msg) => write!(f, "Platform failure: {}", msg),
            BrightnessError::BrightnessOutOfRange(msg) => write!(f, "Brightness out of range: {}", msg),
            BrightnessError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            BrightnessError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
        }
    }
}

impl std::error::Error for BrightnessError {}
