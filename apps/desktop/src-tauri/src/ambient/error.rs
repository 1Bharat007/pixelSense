#[derive(Debug, Clone, PartialEq)]
pub enum AmbientError {
    SensorUnavailable(String),
    ReadFailed(String),
    NotSupported(String),
    InvalidConfiguration(String),
}

impl std::fmt::Display for AmbientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SensorUnavailable(msg) => write!(f, "Sensor unavailable: {}", msg),
            Self::ReadFailed(msg) => write!(f, "Read failed: {}", msg),
            Self::NotSupported(msg) => write!(f, "Not supported: {}", msg),
            Self::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for AmbientError {}
