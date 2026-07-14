use crate::screen_analysis::config::AnalysisConfig;
use crate::screen_analysis::error::ScreenAnalysisError;
use crate::screen_analysis::frame::scaler::RawFrameBuffer;
use crate::screen_analysis::provider::ScreenProvider;

pub struct MockScreenProvider {
    id: String,
    available: bool,
    color: (u8, u8, u8),
}

impl MockScreenProvider {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.into(),
            available: true,
            color: (128, 128, 128),
        }
    }

    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = (r, g, b);
    }
}

impl ScreenProvider for MockScreenProvider {
    fn capture_frame(&self, _display_id: &str, _config: &AnalysisConfig) -> Result<RawFrameBuffer, ScreenAnalysisError> {
        if !self.available {
            return Err(ScreenAnalysisError::CaptureUnavailable("Mock unavailable".into()));
        }
        Ok(RawFrameBuffer {
            pixels: vec![self.color.0, self.color.1, self.color.2, 255, self.color.0, self.color.1, self.color.2, 255],
            width: 2,
            height: 1,
        })
    }

    fn get_provider_id(&self) -> &str {
        &self.id
    }
}
