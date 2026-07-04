use crate::screen_analysis::config::SampleResolution;
use crate::screen_analysis::error::ScreenAnalysisError;

/// Raw pixel buffer with dimensions. Created by the provider; consumed by the analyzer.
/// This struct is intentionally short-lived: it is created, passed to the analyzer,
/// and then immediately dropped. It is never cached, cloned beyond its call scope,
/// or persisted in any form.
///
/// ## Privacy Guarantee
/// No method on this type serializes, logs, or writes pixel data anywhere.
/// The buffer is heap-allocated and released when this struct is dropped.
pub struct RawFrameBuffer {
    /// BGRA pixel data. Each pixel is 4 bytes: [B, G, R, A].
    /// This is the native format of the Windows DXGI output surface.
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl RawFrameBuffer {
    pub fn new(pixels: Vec<u8>, width: u32, height: u32) -> Self {
        Self { pixels, width, height }
    }

    /// Total number of pixels.
    pub fn pixel_count(&self) -> usize {
        (self.width * self.height) as usize
    }
}

/// Downscales a high-resolution raw frame to the target sampling resolution.
///
/// ## Why Downscaling?
/// A 1920×1080 frame contains 2,073,600 pixels. At 4 bytes per pixel that is 8MB.
/// Analyzing all of them per poll is CPU-prohibitive.
/// Downscaling to 64×64 (4,096 pixels, ~16KB) retains statistical accuracy
/// for luminance and histogram analysis while reducing compute by ~500×.
///
/// ## Algorithm
/// Simple area-averaging (box filter). For each output pixel, the average color
/// of the corresponding block of source pixels is computed. This preserves
/// luminance distribution accurately for photometric analysis.
pub struct FrameScaler;

impl FrameScaler {
    /// Downscale `source` to the dimensions specified by `resolution`.
    /// Returns a new `RawFrameBuffer` at the target size.
    /// The source buffer is consumed and dropped within this call.
    pub fn scale(
        source: RawFrameBuffer,
        resolution: &SampleResolution,
    ) -> Result<RawFrameBuffer, ScreenAnalysisError> {
        let (target_w, target_h) = resolution.dimensions();

        if source.width == 0 || source.height == 0 {
            return Err(ScreenAnalysisError::AnalysisFailed(
                "Source frame has zero dimensions".into(),
            ));
        }

        let block_w = source.width as f32 / target_w as f32;
        let block_h = source.height as f32 / target_h as f32;

        let mut output = vec![0u8; (target_w * target_h * 4) as usize];

        for ty in 0..target_h {
            for tx in 0..target_w {
                let src_x_start = (tx as f32 * block_w) as u32;
                let src_y_start = (ty as f32 * block_h) as u32;
                let src_x_end = ((tx as f32 + 1.0) * block_w) as u32;
                let src_y_end = ((ty as f32 + 1.0) * block_h) as u32;

                let (mut sum_b, mut sum_g, mut sum_r) = (0u64, 0u64, 0u64);
                let mut count = 0u64;

                for sy in src_y_start..src_y_end.min(source.height) {
                    for sx in src_x_start..src_x_end.min(source.width) {
                        let idx = ((sy * source.width + sx) * 4) as usize;
                        if idx + 2 < source.pixels.len() {
                            sum_b += source.pixels[idx] as u64;
                            sum_g += source.pixels[idx + 1] as u64;
                            sum_r += source.pixels[idx + 2] as u64;
                            count += 1;
                        }
                    }
                }

                let out_idx = ((ty * target_w + tx) * 4) as usize;
                if count > 0 {
                    output[out_idx] = (sum_b / count) as u8;
                    output[out_idx + 1] = (sum_g / count) as u8;
                    output[out_idx + 2] = (sum_r / count) as u8;
                    output[out_idx + 3] = 255;
                }
            }
        }

        Ok(RawFrameBuffer::new(output, target_w, target_h))
    }
}
