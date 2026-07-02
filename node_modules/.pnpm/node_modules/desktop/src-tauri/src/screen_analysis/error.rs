#[derive(Debug, Clone, PartialEq)]
pub enum ScreenAnalysisError {
    /// The capture provider could not acquire a frame (e.g., no DXGI output found).
    CaptureUnavailable(String),

    /// The display specified by the caller does not exist or has been disconnected.
    DisplayNotFound(String),

    /// The provider does not support the requested region.
    RegionNotSupported(String),

    /// The platform does not support this provider implementation.
    /// Expected on Linux and macOS until native providers are implemented.
    PlatformNotSupported(String),

    /// An unexpected internal error occurred during frame scaling or analysis.
    AnalysisFailed(String),
}

impl std::fmt::Display for ScreenAnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CaptureUnavailable(m) => write!(f, "Capture unavailable: {}", m),
            Self::DisplayNotFound(m) => write!(f, "Display not found: {}", m),
            Self::RegionNotSupported(m) => write!(f, "Region not supported: {}", m),
            Self::PlatformNotSupported(m) => write!(f, "Platform not supported: {}", m),
            Self::AnalysisFailed(m) => write!(f, "Analysis failed: {}", m),
        }
    }
}

impl std::error::Error for ScreenAnalysisError {}
