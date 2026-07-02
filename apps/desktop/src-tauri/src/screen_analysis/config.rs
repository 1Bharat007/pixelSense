/// Configuration for the Screen Analysis Engine.
/// All tuneable parameters are centralized here.
/// Never hardcode analysis constants elsewhere.

#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Target resolution for downscaled frame analysis.
    pub sample_resolution: SampleResolution,

    /// How frequently to poll and analyze a new frame (milliseconds).
    pub poll_interval_ms: u64,

    /// Number of histogram buckets.
    /// More buckets = finer luminance distribution, higher memory cost.
    /// Future: HDR requires more buckets (e.g., 256 for 10-bit content).
    pub histogram_buckets: usize,

    /// Which region of the screen to analyze.
    pub region: RegionOfInterest,

    /// Analysis strategy for this session.
    pub analysis_mode: AnalysisMode,

    /// Whether to attempt GPU-accelerated analysis.
    /// Currently a preparation flag. CPU path is always available as fallback.
    pub gpu_acceleration: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            sample_resolution: SampleResolution::Fixed64x64,
            poll_interval_ms: 500,
            histogram_buckets: 16,
            region: RegionOfInterest::EntireScreen,
            analysis_mode: AnalysisMode::Standard,
            gpu_acceleration: false,
        }
    }
}

/// Controls how the frame is downscaled before analysis.
/// Downscaling must always happen before CPU analysis to stay within budget.
#[derive(Debug, Clone, PartialEq)]
pub enum SampleResolution {
    /// Default. 64×64 pixels. ~4KB buffer. Excellent for luminance + histogram.
    Fixed64x64,

    /// 32×32 pixels. ~1KB buffer. Maximum performance, low detail.
    /// Future: Use for background polling when battery saver is active.
    Performance,

    /// 128×128 pixels. ~16KB buffer. Higher histogram precision.
    /// Future: Use for HDR scene analysis.
    Quality,

    /// Adapts resolution based on current CPU budget.
    /// Future: Implement with a feedback loop from the performance monitor.
    Adaptive,
}

impl SampleResolution {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::Fixed64x64 => (64, 64),
            Self::Performance => (32, 32),
            Self::Quality => (128, 128),
            Self::Adaptive => (64, 64), // Default until adaptive logic is implemented
        }
    }
}

/// Defines which portion of the screen is captured and analyzed.
#[derive(Debug, Clone, PartialEq)]
pub enum RegionOfInterest {
    /// Analyze the full screen surface. Current implementation.
    EntireScreen,

    /// Analyze the center 50% of the screen.
    /// Future: Reduces noise from taskbars and sidebars.
    CenterRegion,

    /// Analyze only the currently focused window.
    /// Future: Requires window enumeration and HWND rect extraction.
    FocusedWindow,

    /// Analyze a user-defined screen rectangle.
    /// Future: Configurable via the Settings page.
    Custom { x: u32, y: u32, width: u32, height: u32 },
}

/// The analysis mode controls which metrics are calculated per frame.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisMode {
    /// Calculate luminance + histogram + visual complexity. Default.
    Standard,

    /// Calculate luminance only. Minimum CPU impact.
    /// Future: Use in emergency mode or when battery is very low.
    LuminanceOnly,

    /// Full analysis including extended metrics for HDR detection.
    /// Future: Activates when an HDR-capable display is detected.
    Extended,
}
