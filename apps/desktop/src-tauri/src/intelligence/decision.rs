use crate::intelligence::models::IntelligenceContext;
use crate::intelligence::confidence::ConfidenceLevel;
use serde::{Deserialize, Serialize};

/// The output of the Decision Engine for one pipeline cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    /// What the engine observed in the current environment.
    pub observation: String,
    /// Why this specific action was chosen.
    pub reason: String,
    /// Confidence 0–100 (percentage from raw score).
    pub confidence: u8,
    /// The human-readable confidence level label.
    pub confidence_label: String,
    /// Expected improvement in comfort (0–100 estimate).
    pub expected_comfort_improvement: u8,
    /// What action will be taken.
    pub action: String,
    /// Target brightness, if adaptation is recommended. None = maintain current.
    pub target_brightness: Option<u8>,
}

/// Context-aware sensitivity multipliers.
/// Gaming and Video sessions should not be interrupted by brightness changes.
fn context_sensitivity(context: &str) -> f32 {
    match context {
        "Gaming" => 0.2,  // Almost never adapt during gaming
        "Video"  => 0.3,  // Rarely adapt during video
        "Design" => 0.7,  // Color-sensitive work — conservative
        "Coding" => 1.0,
        "Reading" => 1.0,
        _ => 0.9,
    }
}

/// The Decision Engine computes *how much* to change brightness given current conditions.
/// 
/// Responsibility: Given a calibrated ComfortProfile and current sensor readings,
/// produce a DecisionRecord with a concrete brightness recommendation.
/// The Adaptation Policy (upstream) has already decided *whether* to adapt.
/// This engine only decides *what* the new target should be.
pub struct DecisionEngine;

impl DecisionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(
        &self, 
        context: &IntelligenceContext, 
        current_brightness: u8,
        profile_opt: Option<crate::configuration::models::ComfortProfile>,
    ) -> DecisionRecord {
        let confidence = ConfidenceLevel::from_score(context.confidence_score);
        let confidence_pct = (context.confidence_score * 100.0).round().clamp(0.0, 100.0) as u8;

        // We no longer strictly gate on ambient confidence, because screen luminance adaptation can always run!
        // Instead, we just apply confidence to the ambient adjustment.

        if let Some(profile) = profile_opt {
            // Context-aware sensitivity multiplier.
            let ctx_mult = context_sensitivity(&context.active_application);
            // Confidence-based sensitivity for ambient sensor only.
            let conf_mult = confidence.sensitivity_multiplier();
            
            // Overall system sensitivity
            let base_sensitivity = profile.sensitivity * ctx_mult;

            // Stage 1 — Ambient Analysis: how much did lux change from our reference?
            let lux_delta = context.current_ambient_lux - profile.reference_lux;
            let ambient_adjustment = lux_delta * (0.05 * base_sensitivity * conf_mult);

            // Stage 2 — Screen Analysis: how bright is the content itself?
            // INVERSE RELATIONSHIP: If the screen content is bright (white window), we DIM the hardware backlight to maintain comfort.
            // If the screen content is dark (dark IDE), we BRIGHTEN the hardware backlight so you can see it.
            let luminance_delta = 50.0 - context.current_screen_luminance;
            let screen_adjustment = luminance_delta * (0.35 * base_sensitivity);

            // Stage 3 — Combine adjustments from reference baseline.
            let mut target_float = profile.reference_brightness as f32
                + ambient_adjustment
                + screen_adjustment;

            // Stage 4 — Apply profile brightness limits.
            target_float = target_float.clamp(
                profile.min_brightness as f32,
                profile.max_brightness as f32,
            );

            // Stage 5 — Apply minimum change threshold (suppress micro-corrections).
            let target = target_float.round() as u8;
            let diff = (target as i32 - current_brightness as i32).abs();
            if diff < 3 {
                return DecisionRecord {
                    observation: "Environment matches comfort profile within tolerance.".into(),
                    reason: format!(
                        "Difference of {}% is below threshold — no change needed.",
                        diff
                    ),
                    confidence: confidence_pct,
                    confidence_label: confidence.label().into(),
                    expected_comfort_improvement: 0,
                    action: "Maintain current brightness".into(),
                    target_brightness: None,
                };
            }

            // Stage 6 — Emit recommendation with precise observation.
            let is_screen_driven = screen_adjustment.abs() > ambient_adjustment.abs();

            let (observation, reason) = if is_screen_driven {
                if context.current_screen_luminance > 65.0 {
                    (
                        format!("Bright screen content detected ({:.0}% luminance).", context.current_screen_luminance),
                        "Reducing backlight brightness to prevent sudden eye glare.".into(),
                    )
                } else if context.current_screen_luminance < 35.0 {
                    (
                        format!("Dark screen content detected ({:.0}% luminance).", context.current_screen_luminance),
                        "Increasing backlight brightness to ensure dark text and UI elements remain readable.".into(),
                    )
                } else {
                    (
                        format!("Screen content luminance is {:.0}%.", context.current_screen_luminance),
                        "Adjusting brightness to maintain perceived visual comfort.".into(),
                    )
                }
            } else {
                if context.current_ambient_lux > profile.reference_lux + 20.0 {
                    (
                        format!("Room is brighter than reference ({:.0} lux vs {:.0} lux baseline).", context.current_ambient_lux, profile.reference_lux),
                        "Increasing brightness to reduce eye strain from ambient glare.".into(),
                    )
                } else {
                    (
                        format!("Room is darker than reference ({:.0} lux vs {:.0} lux baseline).", context.current_ambient_lux, profile.reference_lux),
                        "Reducing brightness to prevent glare and eye fatigue.".into(),
                    )
                }
            };

            DecisionRecord {
                observation,
                reason,
                confidence: confidence_pct,
                confidence_label: confidence.label().into(),
                expected_comfort_improvement: (diff.min(20)) as u8,
                action: if target > current_brightness {
                    format!("Increase brightness {} → {}%", current_brightness, target)
                } else {
                    format!("Reduce brightness {} → {}%", current_brightness, target)
                },
                target_brightness: Some(target),
            }
        } else {
            // No calibration profile — Manual Adaptive Mode.
            // Do NOT make automated changes without a baseline.
            DecisionRecord {
                observation: "No calibration profile found.".into(),
                reason: "Manual Adaptive Mode — set a brightness you like, then enable Protection to learn from it.".into(),
                confidence: confidence_pct,
                confidence_label: confidence.label().into(),
                expected_comfort_improvement: 0,
                action: "Waiting for calibration".into(),
                target_brightness: None,
            }
        }
    }
}

impl Default for DecisionEngine {
    fn default() -> Self {
        Self::new()
    }
}
