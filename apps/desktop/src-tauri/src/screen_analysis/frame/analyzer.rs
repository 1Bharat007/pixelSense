use crate::screen_analysis::config::AnalysisConfig;
use crate::screen_analysis::error::ScreenAnalysisError;
use crate::screen_analysis::frame::scaler::RawFrameBuffer;
use crate::screen_analysis::models::{FrameMetrics, LuminanceHistogram, VisualComplexity};

/// Analyzes a downscaled frame buffer and produces all screen metrics.
///
/// ## Responsibilities
/// - Calculate average, peak, and minimum luminance using the standard photometric formula.
/// - Calculate luminance standard deviation (used for VisualComplexity classification).
/// - Calculate white and black pixel percentages.
/// - Build the luminance histogram.
/// - Classify VisualComplexity from the standard deviation.
///
/// ## Non-Responsibilities
/// - Does NOT communicate with hardware.
/// - Does NOT decide brightness.
/// - Does NOT cache, serialize, or log pixel data.
///
/// ## Photometric Formula Used
/// Relative luminance (Y) per pixel using the IEC 61966-2-1 sRGB standard:
///   Y = 0.2126 × R_linear + 0.7152 × G_linear + 0.0722 × B_linear
/// where R_linear = (R/255)^2.2 approximation for gamma expansion.
/// For computational efficiency at the polling rate, a simplified fast-path is used:
///   Y_approx = (0.299×R + 0.587×G + 0.114×B) / 255 × 100
/// This is the ITU-R BT.601 luma approximation. Accurate enough for comfort decisions.
/// A more precise sRGB path is documented for future HDR/Wide-Gamut support.
pub struct ScreenAnalyzer;

impl ScreenAnalyzer {
    /// Analyze the given downscaled buffer and return all frame metrics.
    /// The buffer is consumed and dropped at the end of this call.
    pub fn analyze(
        frame: RawFrameBuffer,
        config: &AnalysisConfig,
    ) -> Result<FrameMetrics, ScreenAnalysisError> {
        let pixel_count = frame.pixel_count();
        if pixel_count == 0 {
            return Err(ScreenAnalysisError::AnalysisFailed(
                "Empty frame buffer".into(),
            ));
        }

        let bucket_count = config.histogram_buckets;
        let mut histogram = LuminanceHistogram::new(bucket_count);

        let mut sum_lum = 0.0f64;
        let mut peak = 0.0f32;
        let mut min_lum = 100.0f32;
        let mut bright_count = 0usize;
        let mut dark_count = 0usize;

        // First pass: compute per-pixel luminance, accumulate sums.
        let mut lum_values: Vec<f32> = Vec::with_capacity(pixel_count);

        for i in 0..pixel_count {
            let base = i * 4;
            if base + 2 >= frame.pixels.len() {
                break;
            }
            let b = frame.pixels[base] as f32;
            let g = frame.pixels[base + 1] as f32;
            let r = frame.pixels[base + 2] as f32;

            // ITU-R BT.601 luma (normalized to 0–100 range)
            let lum = (0.299 * r + 0.587 * g + 0.114 * b) / 255.0 * 100.0;

            lum_values.push(lum);
            sum_lum += lum as f64;

            if lum > peak {
                peak = lum;
            }
            if lum < min_lum {
                min_lum = lum;
            }
            if lum > 80.0 {
                bright_count += 1;
            }
            if lum < 20.0 {
                dark_count += 1;
            }

            // Assign to histogram bucket
            let bucket_idx = ((lum / 100.0) * (bucket_count - 1) as f32) as usize;
            let bucket_idx = bucket_idx.min(bucket_count - 1);
            histogram.buckets[bucket_idx] += 1.0;
        }

        let count = lum_values.len() as f64;
        let average = (sum_lum / count) as f32;

        // Normalize histogram to fractions summing to 1.0
        for bucket in histogram.buckets.iter_mut() {
            *bucket /= count as f32;
        }

        // Second pass: standard deviation (for VisualComplexity)
        let variance: f64 = lum_values
            .iter()
            .map(|&l| {
                let diff = l as f64 - sum_lum / count;
                diff * diff
            })
            .sum::<f64>()
            / count;
        let std_dev = variance.sqrt() as f32;

        let visual_complexity = VisualComplexity::from_luminance_std_dev(std_dev);

        Ok(FrameMetrics {
            average_luminance: average,
            peak_luminance: peak,
            min_luminance: min_lum,
            luminance_std_dev: std_dev,
            white_percentage: bright_count as f32 / pixel_count as f32 * 100.0,
            black_percentage: dark_count as f32 / pixel_count as f32 * 100.0,
            histogram,
            visual_complexity,
        })
    }
}
