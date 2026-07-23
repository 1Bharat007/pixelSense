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
        // Store in BGRA order (b, g, r) for DXGI output surface compatibility
        self.color = (b, g, r);
    }
}

impl ScreenProvider for MockScreenProvider {
    fn capture_frame(&self, _display_id: &str, config: &AnalysisConfig) -> Result<RawFrameBuffer, ScreenAnalysisError> {
        if !self.available {
            return Err(ScreenAnalysisError::CaptureUnavailable("Mock unavailable".into()));
        }
        let (target_w, target_h) = config.sample_resolution.dimensions();
        let width = target_w.max(128);
        let height = target_h.max(128);
        let total_pixels = (width * height) as usize;
        let mut pixels = Vec::with_capacity(total_pixels * 4);
        for _ in 0..total_pixels {
            pixels.push(self.color.0); // B
            pixels.push(self.color.1); // G
            pixels.push(self.color.2); // R
            pixels.push(255);          // A
        }
        Ok(RawFrameBuffer {
            pixels,
            width,
            height,
        })
    }

    fn get_provider_id(&self) -> &str {
        &self.id
    }
}
