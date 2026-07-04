use crate::screen_analysis::config::AnalysisConfig;
use crate::screen_analysis::error::ScreenAnalysisError;
use crate::screen_analysis::frame::scaler::RawFrameBuffer;

/// Abstract interface for screen capture providers.
///
/// Each platform implements this trait independently.
/// The `ScreenAnalysisManager` only interacts with this trait,
/// never with platform-specific types directly.
///
/// ## Provider Responsibilities
/// - Capture a single frame from the specified display.
/// - Return the raw pixel buffer for downstream analysis.
/// - Release all hardware resources (surfaces, GPU memory) before returning.
///
/// ## Provider Non-Responsibilities
/// - Must NOT analyze, cache, serialize, or log pixel data.
/// - Must NOT communicate with the UI or any other subsystem.
/// - Must NOT make brightness decisions.
pub trait ScreenProvider: Send + Sync {
    fn capture_frame(
        &self,
        display_id: &str,
        config: &AnalysisConfig,
    ) -> Result<RawFrameBuffer, ScreenAnalysisError>;

    fn get_provider_id(&self) -> &str;
}
