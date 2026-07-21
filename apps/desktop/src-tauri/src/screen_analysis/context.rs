use crate::screen_analysis::models::{FrameMetrics, VisualComplexity};

pub fn detect_context(metrics: &FrameMetrics, previous_metrics: Option<&FrameMetrics>, active_app: &str) -> String {
    let app = active_app.to_lowercase();
    
    // 1. Process Name Strong Hints
    if app.contains("code") || app.contains("devenv") || app.contains("rider") || app.contains("idea") {
        return "Coding".into();
    }
    
    if app.contains("vlc") || app.contains("mpc") || app.contains("netflix") {
        return "Video".into();
    }
    
    if app.contains("photoshop") || app.contains("illustrator") || app.contains("figma") || app.contains("premiere") {
        return "Design".into();
    }
    
    // Check for games (simple heuristics)
    if app.contains("game") || app.contains("steam") || app.contains("epic") {
        return "Gaming".into();
    }

    // 2. Browser checks (can be reading or video)
    if app.contains("chrome") || app.contains("firefox") || app.contains("edge") || app.contains("brave") {
        // If high motion, probably video on browser
        if let Some(prev) = previous_metrics {
            let diff = (metrics.average_luminance - prev.average_luminance).abs();
            if diff > 15.0 || metrics.visual_complexity == VisualComplexity::VeryHigh {
                return "Video".into();
            }
        }
        
        // Otherwise, assume reading / browsing
        return "Reading".into();
    }

    // 3. Fallback to pure screen metrics
    if let Some(prev) = previous_metrics {
        let diff = (metrics.average_luminance - prev.average_luminance).abs();
        if diff > 20.0 || metrics.visual_complexity == VisualComplexity::VeryHigh {
            // High motion or rapid changes
            if metrics.contrast_estimation > 50.0 {
                return "Gaming".into();
            }
            return "Video".into();
        }
    }

    if metrics.white_percentage > 60.0 {
        return "Reading".into();
    }

    if metrics.black_percentage > 70.0 && metrics.visual_complexity == VisualComplexity::VeryLow {
        return "Coding".into();
    }

    "Desktop".into()
}
