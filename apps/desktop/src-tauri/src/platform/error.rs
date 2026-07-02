use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlatformError {
    UnsupportedPlatform(String),
    NotImplemented(String),
    NativeApiUnavailable(String),
    InvalidConfigurationPath(String),
    NotificationUnavailable(String),
}

impl std::fmt::Display for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformError::UnsupportedPlatform(msg) => write!(f, "Unsupported platform: {}", msg),
            PlatformError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            PlatformError::NativeApiUnavailable(msg) => write!(f, "Native API unavailable: {}", msg),
            PlatformError::InvalidConfigurationPath(msg) => write!(f, "Invalid configuration path: {}", msg),
            PlatformError::NotificationUnavailable(msg) => write!(f, "Notification unavailable: {}", msg),
        }
    }
}

impl std::error::Error for PlatformError {}
