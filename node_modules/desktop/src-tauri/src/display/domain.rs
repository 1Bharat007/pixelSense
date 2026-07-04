use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisplayInfo {
    pub id: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: Option<f32>,
    pub is_primary: bool,
    pub capabilities: DisplayCapabilities,
}

/// Represents the operations supported by a display.
///
/// **Architectural Compromise**: This model currently resides within the `display::domain`.
/// In a future major refactor, Capability will become its own dedicated domain.
/// Future extensibility is planned for: Variable Refresh Rate, Color Temperature,
/// Contrast, Gamma, and Ambient Sensor capabilities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DisplayCapabilities {
    pub brightness: bool,
    pub hdr: bool,
    pub ddc_ci: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisplayError {
    NotFound(String),
    PlatformError(String),
    Unknown(String),
}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::NotFound(msg) => write!(f, "Display not found: {}", msg),
            DisplayError::PlatformError(msg) => write!(f, "Platform error: {}", msg),
            DisplayError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for DisplayError {}

