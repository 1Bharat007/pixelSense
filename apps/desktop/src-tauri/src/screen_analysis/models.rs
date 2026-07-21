use std::time::{SystemTime, UNIX_EPOCH};

/// Normalized luminance histogram across N buckets.
///
/// The histogram records what fraction of analyzed pixels fall into each
/// luminance bucket. This data serves many future use cases:
///
/// - **Flash detection**: A bimodal histogram (spikes at 0 and 100) indicates strobing content.
/// - **HDR analysis**: A long-tailed distribution indicates HDR-like content.
/// - **Exposure estimation**: Histogram shape reveals under/over-exposure.
/// - **Contrast analysis**: Distance between histogram peaks indicates contrast ratio.
/// - **White balance**: Combined with RGB channels, reveals color temperature.
/// - **Dynamic range**: Max bucket index minus min populated bucket index.
#[derive(Debug, Clone)]
pub struct LuminanceHistogram {
    /// Normalized fraction of pixels per bucket. Values sum to 1.0.
    pub buckets: Vec<f32>,
    /// Number of buckets.
    pub bucket_count: usize,
}

impl LuminanceHistogram {
    pub fn new(bucket_count: usize) -> Self {
        Self {
            buckets: vec![0.0; bucket_count],
            bucket_count,
        }
    }
}

/// Qualitative measure of how much visual change exists across the frame.
///
/// This metric will become a key input for future adaptive algorithms:
/// - A **dark IDE** has very low complexity. PixelSense should apply gentle adjustments.
/// - A **white PDF** has low-medium complexity. Predictable, stable adjustments.
/// - A **browser with mixed content** has medium complexity. Moderate adjustment.
/// - A **video with rapid motion or scene changes** has high complexity.
///   PixelSense should dampen reactions to avoid visual noise.
/// - An **explosion or strobe scene** has very high complexity.
///   PixelSense should hold its adjustment and wait for stability.
#[derive(Debug, Clone, PartialEq)]
pub enum VisualComplexity {
    VeryLow,   // e.g., Dark code editor, terminal
    Low,       // e.g., White document, static webpage
    Medium,    // e.g., Browser with mixed content, news site
    High,      // e.g., YouTube video, game with moderate motion
    VeryHigh,  // e.g., Action scene, rapid screen changes, strobe content
}

impl VisualComplexity {
    /// Classify complexity from a standard deviation of per-pixel luminance values.
    /// Higher deviation = more visual variety = higher complexity.
    pub fn from_luminance_std_dev(std_dev: f32) -> Self {
        if std_dev < 5.0 {
            Self::VeryLow
        } else if std_dev < 15.0 {
            Self::Low
        } else if std_dev < 30.0 {
            Self::Medium
        } else if std_dev < 50.0 {
            Self::High
        } else {
            Self::VeryHigh
        }
    }
}

/// Normalized metrics extracted from a single analyzed frame.
/// All values are in the 0.0–100.0 range unless otherwise noted.
#[derive(Debug, Clone)]
pub struct FrameMetrics {
    /// Average perceived luminance across all sampled pixels. (0.0 = black, 100.0 = white)
    pub average_luminance: f32,

    /// Maximum single-pixel luminance in the sample. Indicates bright highlights.
    pub peak_luminance: f32,

    /// Minimum single-pixel luminance. Indicates deep shadows.
    pub min_luminance: f32,

    /// Standard deviation of luminance values. High = high contrast scene.
    pub luminance_std_dev: f32,

    /// Median luminance of the sample.
    pub median_luminance: f32,

    /// Estimated contrast ratio (derived from peak, min, and average).
    pub contrast_estimation: f32,

    /// Percentage of pixels considered "bright" (luminance > 80.0).
    pub white_percentage: f32,

    /// Percentage of pixels considered "dark" (luminance < 20.0).
    pub black_percentage: f32,

    /// Luminance histogram — distribution across N buckets.
    pub histogram: LuminanceHistogram,

    /// Qualitative visual complexity classification.
    pub visual_complexity: VisualComplexity,
}

/// The complete output of one Screen Analysis Engine cycle.
/// This is the single object passed to AdaptiveBrightnessService.
/// It contains everything needed to make a comfort recommendation.
#[derive(Debug, Clone)]
pub struct ScreenAnalysisResult {
    /// Display identifier this result was captured from.
    pub display_id: String,

    /// Core frame metrics.
    pub metrics: FrameMetrics,

    /// The region that was analyzed.
    pub analyzed_region: String,

    /// Whether this result is considered reliable.
    pub is_reliable: bool,

    /// UNIX timestamp in milliseconds when capture occurred.
    pub timestamp_ms: u64,

    /// Elapsed time for the full capture + analysis cycle in milliseconds.
    pub analysis_duration_ms: u64,
}

impl ScreenAnalysisResult {
    pub fn timestamp_now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}
