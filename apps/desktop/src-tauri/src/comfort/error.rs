use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComfortError {
    StorageFailure(String),
    ProfileNotFound(String),
    InvalidProfile(String),
    MatchingFailed(String),
}

impl std::fmt::Display for ComfortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComfortError::StorageFailure(msg) => write!(f, "Storage failure: {}", msg),
            ComfortError::ProfileNotFound(msg) => write!(f, "Profile not found: {}", msg),
            ComfortError::InvalidProfile(msg) => write!(f, "Invalid profile: {}", msg),
            ComfortError::MatchingFailed(msg) => write!(f, "Matching failed: {}", msg),
        }
    }
}

impl std::error::Error for ComfortError {}
