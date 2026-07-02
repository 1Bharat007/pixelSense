use crate::screen_analysis::config::AnalysisConfig;
use crate::screen_analysis::error::ScreenAnalysisError;
use crate::screen_analysis::frame::scaler::RawFrameBuffer;
use crate::screen_analysis::provider::ScreenProvider;

pub struct MacosScreenProvider;

impl ScreenProvider for MacosScreenProvider {
    fn capture_frame(&self, _display_id: &str, _config: &AnalysisConfig) -> Result<RawFrameBuffer, ScreenAnalysisError> {
        Err(ScreenAnalysisError::PlatformNotSupported(
            "macOS screen capture not yet implemented. Planned via ScreenCaptureKit.".into(),
        ))
    }

    fn get_provider_id(&self) -> &str { "macos_stub" }
}
