use crate::screen_analysis::config::AnalysisConfig;
use crate::screen_analysis::error::ScreenAnalysisError;
use crate::screen_analysis::frame::scaler::RawFrameBuffer;
use crate::screen_analysis::provider::ScreenProvider;
use crate::platform::hardware::dxgi::capture::DuplicationSession;
use std::sync::Mutex;

pub struct WindowsScreenProvider {
    session: Mutex<DuplicationSession>,
}

impl WindowsScreenProvider {
    pub fn new() -> Self {
        Self {
            session: Mutex::new(DuplicationSession::new()),
        }
    }
}

impl ScreenProvider for WindowsScreenProvider {
    fn capture_frame(&self, _display_id: &str, _config: &AnalysisConfig) -> Result<RawFrameBuffer, ScreenAnalysisError> {
        let mut session = self.session.lock().unwrap();
        session.capture_frame().map_err(|e| ScreenAnalysisError::CaptureUnavailable(e.to_string()))
    }

    fn get_provider_id(&self) -> &str {
        "windows_dxgi"
    }
}
