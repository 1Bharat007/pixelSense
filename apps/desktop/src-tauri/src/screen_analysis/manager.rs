use crate::screen_analysis::config::AnalysisConfig;
use crate::screen_analysis::error::ScreenAnalysisError;
use crate::screen_analysis::frame::analyzer::ScreenAnalyzer;
use crate::screen_analysis::frame::scaler::FrameScaler;
use crate::screen_analysis::models::ScreenAnalysisResult;
use crate::screen_analysis::provider::ScreenProvider;
use std::time::Instant;

/// Orchestrates the Screen Analysis Engine pipeline.
///
/// ## Pipeline
/// ```text
/// ScreenProvider::capture_frame()
///     ↓
/// FrameScaler::scale()        [downscale to sample resolution]
///     ↓
/// ScreenAnalyzer::analyze()   [luminance + histogram + visual complexity]
///     ↓
/// ScreenAnalysisResult        [output — no pixel data survives]
///     ↓
/// frame buffer dropped
/// ```
///
/// ## Responsibilities
/// - Poll the provider on the configured interval.
/// - Pass the raw frame through the analysis pipeline.
/// - Return a `ScreenAnalysisResult` containing only numeric metrics.
///
/// ## Non-Responsibilities
/// - Does NOT modify brightness.
/// - Does NOT communicate with TransitionEngine or BrightnessEngine.
/// - Does NOT cache frames or pixel data between polls.
/// - Does NOT make comfort decisions.
pub struct ScreenAnalysisManager {
    config: AnalysisConfig,
    provider: Box<dyn ScreenProvider>,
}

impl ScreenAnalysisManager {
    pub fn new(config: AnalysisConfig, provider: Box<dyn ScreenProvider>) -> Self {
        Self { config, provider }
    }

    /// Run one full analysis cycle for the given display.
    ///
    /// The pixel buffer captured by the provider is scaled, analyzed, and then dropped
    /// within this call. No pixel data escapes this function.
    pub fn analyze_display(&self, display_id: &str) -> Result<ScreenAnalysisResult, ScreenAnalysisError> {
        let started_at = Instant::now();
        let timestamp_ms = ScreenAnalysisResult::timestamp_now();

        // Step 1: Capture raw frame from platform provider.
        let raw_frame = self.provider.capture_frame(display_id, &self.config)?;

        // Step 2: Downscale to configured sample resolution.
        let scaled_frame = FrameScaler::scale(raw_frame, &self.config.sample_resolution)?;

        // Step 3: Analyze — compute all metrics. scaled_frame is consumed and dropped here.
        let metrics = ScreenAnalyzer::analyze(scaled_frame, &self.config)?;

        let analysis_duration_ms = started_at.elapsed().as_millis() as u64;

        Ok(ScreenAnalysisResult {
            display_id: display_id.to_string(),
            metrics,
            analyzed_region: format!("{:?}", self.config.region),
            is_reliable: true,
            timestamp_ms,
            analysis_duration_ms,
        })
    }

    pub fn get_config(&self) -> &AnalysisConfig {
        &self.config
    }
}
