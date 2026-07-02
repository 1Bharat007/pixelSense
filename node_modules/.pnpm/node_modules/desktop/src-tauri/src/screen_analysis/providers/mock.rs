use crate::screen_analysis::config::AnalysisConfig;
use crate::screen_analysis::error::ScreenAnalysisError;
use crate::screen_analysis::frame::scaler::RawFrameBuffer;
use crate::screen_analysis::provider::ScreenProvider;
use std::sync::Mutex;

/// Mock screen provider for unit testing.
///
/// Produces configurable synthetic pixel buffers that simulate real screen conditions.
/// Used in all unit and integration tests to validate the analysis pipeline
/// without requiring a real display or GPU.
pub struct MockScreenProvider {
    pub provider_id: String,
    /// RGB value for all pixels (simulates a uniform screen).
    pub mock_color: Mutex<(u8, u8, u8)>,
    pub is_available: Mutex<bool>,
}

impl MockScreenProvider {
    pub fn new(id: &str) -> Self {
        Self {
            provider_id: id.into(),
            mock_color: Mutex::new((128, 128, 128)), // default: mid-grey
            is_available: Mutex::new(true),
        }
    }

    /// Set the uniform RGB color all pixels will report.
    pub fn set_color(&self, r: u8, g: u8, b: u8) {
        *self.mock_color.lock().unwrap() = (r, g, b);
    }

    pub fn set_available(&self, available: bool) {
        *self.is_available.lock().unwrap() = available;
    }
}

impl ScreenProvider for MockScreenProvider {
    fn capture_frame(
        &self,
        _display_id: &str,
        config: &AnalysisConfig,
    ) -> Result<RawFrameBuffer, ScreenAnalysisError> {
        if !*self.is_available.lock().unwrap() {
            return Err(ScreenAnalysisError::CaptureUnavailable(
                "Mock provider disabled".into(),
            ));
        }

        let (w, h) = config.sample_resolution.dimensions();
        let (r, g, b) = *self.mock_color.lock().unwrap();

        // BGRA format (matching Windows DXGI surface format)
        let pixels: Vec<u8> = (0..(w * h))
            .flat_map(|_| vec![b, g, r, 255u8])
            .collect();

        Ok(RawFrameBuffer::new(pixels, w, h))
    }

    fn get_provider_id(&self) -> &str {
        &self.provider_id
    }
}
