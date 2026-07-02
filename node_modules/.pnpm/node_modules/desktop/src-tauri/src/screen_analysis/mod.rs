pub mod config;
pub mod error;
pub mod factory;
pub mod frame;
pub mod manager;
pub mod models;
pub mod provider;

#[cfg(test)]
mod tests {
    use crate::screen_analysis::config::{AnalysisConfig, RegionOfInterest, SampleResolution};
    use crate::screen_analysis::error::ScreenAnalysisError;
    use crate::screen_analysis::manager::ScreenAnalysisManager;
    use crate::screen_analysis::models::VisualComplexity;
    use crate::screen_analysis::providers::mock::MockScreenProvider;

    fn make_manager(color: (u8, u8, u8), resolution: SampleResolution) -> ScreenAnalysisManager {
        let provider = Box::new(MockScreenProvider::new("test"));
        provider.set_color(color.0, color.1, color.2);
        ScreenAnalysisManager::new(
            AnalysisConfig {
                sample_resolution: resolution,
                ..Default::default()
            },
            provider,
        )
    }

    #[test]
    fn test_black_frame_luminance() {
        let manager = make_manager((0, 0, 0), SampleResolution::Fixed64x64);
        let result = manager.analyze_display("test_disp").unwrap();
        // All-black frame: average luminance should be 0
        assert!(result.metrics.average_luminance < 1.0, "Expected near-zero luminance");
        assert!(result.metrics.black_percentage > 99.0);
    }

    #[test]
    fn test_white_frame_luminance() {
        let manager = make_manager((255, 255, 255), SampleResolution::Fixed64x64);
        let result = manager.analyze_display("test_disp").unwrap();
        // All-white frame: average luminance should be ~100
        assert!(result.metrics.average_luminance > 99.0, "Expected near-100 luminance");
        assert!(result.metrics.white_percentage > 99.0);
    }

    #[test]
    fn test_mid_grey_luminance() {
        let manager = make_manager((128, 128, 128), SampleResolution::Fixed64x64);
        let result = manager.analyze_display("test_disp").unwrap();
        // Mid-grey should be around 50
        let lum = result.metrics.average_luminance;
        assert!(lum > 45.0 && lum < 55.0, "Expected ~50 luminance, got {}", lum);
    }

    #[test]
    fn test_visual_complexity_uniform_frame_is_very_low() {
        let manager = make_manager((128, 128, 128), SampleResolution::Fixed64x64);
        let result = manager.analyze_display("test_disp").unwrap();
        // Uniform grey has zero std dev → VeryLow complexity
        assert_eq!(result.metrics.visual_complexity, VisualComplexity::VeryLow);
    }

    #[test]
    fn test_histogram_populated() {
        let manager = make_manager((128, 128, 128), SampleResolution::Fixed64x64);
        let result = manager.analyze_display("test_disp").unwrap();
        // All pixels should be in one bucket; histogram sum should be ~1.0
        let bucket_sum: f32 = result.metrics.histogram.buckets.iter().sum();
        assert!((bucket_sum - 1.0).abs() < 0.01, "Histogram should sum to 1.0, got {}", bucket_sum);
    }

    #[test]
    fn test_capture_unavailable_no_panic() {
        let provider = Box::new(MockScreenProvider::new("test_unavailable"));
        provider.set_available(false);
        let manager = ScreenAnalysisManager::new(AnalysisConfig::default(), provider);
        let result = manager.analyze_display("test_disp");
        assert!(matches!(result, Err(ScreenAnalysisError::CaptureUnavailable(_))));
    }

    #[test]
    fn test_performance_resolution() {
        let manager = make_manager((200, 100, 50), SampleResolution::Performance);
        let result = manager.analyze_display("test_disp").unwrap();
        // Should complete and return a valid result at 32x32
        assert!(result.metrics.average_luminance >= 0.0);
        assert!(result.analysis_duration_ms < 100); // Should be very fast
    }

    #[test]
    fn test_region_of_interest_reflected_in_result() {
        let provider = Box::new(MockScreenProvider::new("roi_test"));
        let manager = ScreenAnalysisManager::new(
            AnalysisConfig {
                region: RegionOfInterest::CenterRegion,
                ..Default::default()
            },
            provider,
        );
        let result = manager.analyze_display("test_disp").unwrap();
        assert!(result.analyzed_region.contains("CenterRegion"));
    }

    #[test]
    fn test_result_has_timestamp() {
        let manager = make_manager((100, 100, 100), SampleResolution::Fixed64x64);
        let result = manager.analyze_display("test_disp").unwrap();
        assert!(result.timestamp_ms > 0);
    }
}
