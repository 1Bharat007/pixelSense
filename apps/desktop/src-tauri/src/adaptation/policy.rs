use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// The context passed into the Adaptation Policy for a decision.
pub struct PolicyContext {
    /// Current ambient lux reading.
    pub current_lux: f32,
    /// Current screen luminance (0–100).
    pub current_luminance: f32,
    /// Detected application context (e.g. "Gaming", "Video", "Coding").
    pub app_context: String,
    /// Raw confidence score from sensor [0.0, 1.0].
    pub confidence: f32,
    /// Whether a manual override is currently in force.
    pub manual_override_active: bool,
    /// Whether the foreground window is fullscreen.
    pub is_fullscreen: bool,
}

/// The outcome from the Adaptation Policy.
#[derive(Debug, Clone)]
pub enum AdaptationDecision {
    /// Proceed with brightness adaptation.
    Adapt { reason: String },
    /// Do not change brightness right now. Reason is logged to EventLog.
    Skip { reason: String },
}

impl AdaptationDecision {
    pub fn is_adapt(&self) -> bool {
        matches!(self, Self::Adapt { .. })
    }
    pub fn reason(&self) -> &str {
        match self {
            Self::Adapt { reason } | Self::Skip { reason } => reason,
        }
    }
}

/// Minimum lux change required before we consider adapting.
const MIN_LUX_DELTA: f32 = 5.0;
/// Minimum luminance change required before we consider adapting.
const MIN_LUMINANCE_DELTA: f32 = 5.0;
/// Number of consecutive stable readings before adapting (noise filter).
const STABILITY_WINDOW: usize = 3;
/// Duration a context must be stable before we respond to it.
const CONTEXT_STABILITY_MS: u64 = 500;

/// The Adaptation Policy is the "should we adapt?" gate.
///
/// It sits between the Decision Engine (which computes *how much* to change)
/// and the Transition Engine (which *executes* the change).
///
/// Responsibility: Given current conditions, decide whether adaptation is
/// appropriate *right now*. This eliminates oscillation, noise reactions,
/// and interruptions during immersive sessions.
pub struct AdaptationPolicy {
    /// Ring buffer of recent lux readings for stability detection.
    lux_history: VecDeque<f32>,
    /// Ring buffer of recent luminance readings.
    luminance_history: VecDeque<f32>,
    /// The lux value at the time the last adaptation was approved.
    last_adapted_lux: Option<f32>,
    /// The luminance at the time the last adaptation was approved.
    last_adapted_luminance: Option<f32>,
    /// When the current context was first detected (for stability gating).
    context_stable_since: Option<(String, Instant)>,
}

impl AdaptationPolicy {
    pub fn new() -> Self {
        Self {
            lux_history: VecDeque::with_capacity(STABILITY_WINDOW + 2),
            luminance_history: VecDeque::with_capacity(STABILITY_WINDOW + 2),
            last_adapted_lux: None,
            last_adapted_luminance: None,
            context_stable_since: None,
        }
    }

    /// Record a new observation. Must be called on every pipeline tick.
    pub fn observe(&mut self, lux: f32, luminance: f32) {
        if self.lux_history.len() >= STABILITY_WINDOW + 2 {
            self.lux_history.pop_front();
        }
        if self.luminance_history.len() >= STABILITY_WINDOW + 2 {
            self.luminance_history.pop_front();
        }
        self.lux_history.push_back(lux);
        self.luminance_history.push_back(luminance);
    }

    /// Core decision gate. Call after `observe()`.
    pub fn should_adapt(&mut self, ctx: &PolicyContext) -> AdaptationDecision {
        // Rule 1: Manual override — always respect it.
        if ctx.manual_override_active {
            return AdaptationDecision::Skip {
                reason: "Manual override is active".into(),
            };
        }

        // Rule 2: Pause during fullscreen gaming or video — don't interrupt immersive sessions.
        if ctx.is_fullscreen && (ctx.app_context == "Gaming" || ctx.app_context == "Video") {
            return AdaptationDecision::Skip {
                reason: format!("Fullscreen {} session active — pausing adaptation", ctx.app_context),
            };
        }

        // Rule 3: Check if we have a meaningful screen luminance change or a valid ambient lux change.
        use crate::intelligence::confidence::ConfidenceLevel;
        let ambient_valid = ConfidenceLevel::from_score(ctx.confidence).should_adapt();
        
        let screen_changed = match self.last_adapted_luminance {
            Some(last_lum) => (ctx.current_luminance - last_lum).abs() >= MIN_LUMINANCE_DELTA,
            None => true,
        };

        let lux_changed = match self.last_adapted_lux {
            Some(last_lux) if ambient_valid => (ctx.current_lux - last_lux).abs() >= MIN_LUX_DELTA,
            _ => false,
        };

        // If neither ambient nor screen has changed meaningfully, or if screen volatility is too high, skip.
        if !screen_changed && !lux_changed {
            return AdaptationDecision::Skip {
                reason: "Environment and screen content within comfort tolerance".into(),
            };
        }

        // Rule 4: Screen Volatility Check — ignore temporary bright flashes (transient content)
        let lum_variance = self.luminance_variance();
        if lum_variance > 20.0 {
            return AdaptationDecision::Skip {
                reason: format!(
                    "Screen luminance is volatile (σ={:.1}) — likely transient content",
                    lum_variance.sqrt()
                ),
            };
        }

        // Rule 5: Context must be stable for at least CONTEXT_STABILITY_MS before acting.
        let context_stable = match &self.context_stable_since {
            Some((last_ctx, since)) if last_ctx == &ctx.app_context => {
                since.elapsed() >= Duration::from_millis(CONTEXT_STABILITY_MS)
            }
            _ => {
                // Context just changed — reset stability timer.
                self.context_stable_since = Some((ctx.app_context.clone(), Instant::now()));
                false
            }
        };

        if !context_stable {
            return AdaptationDecision::Skip {
                reason: format!(
                    "Context '{}' not yet stable — waiting {}ms",
                    ctx.app_context, CONTEXT_STABILITY_MS
                ),
            };
        }

        // All gates passed — approve adaptation.
        self.last_adapted_lux = Some(ctx.current_lux);
        self.last_adapted_luminance = Some(ctx.current_luminance);

        AdaptationDecision::Adapt {
            reason: format!(
                "Stable conditions in '{}' context (confidence: {:.0}%)",
                ctx.app_context,
                ctx.confidence * 100.0
            ),
        }
    }

    #[allow(dead_code)]
    fn lux_variance(&self) -> f32 {
        if self.lux_history.len() < 2 {
            return 0.0;
        }
        let mean = self.lux_history.iter().sum::<f32>() / self.lux_history.len() as f32;
        let variance = self.lux_history.iter().map(|&x| (x - mean).powi(2)).sum::<f32>()
            / self.lux_history.len() as f32;
        variance
    }

    fn luminance_variance(&self) -> f32 {
        if self.luminance_history.len() < 2 {
            return 0.0;
        }
        let mean = self.luminance_history.iter().sum::<f32>() / self.luminance_history.len() as f32;
        let variance = self.luminance_history.iter().map(|&x| (x - mean).powi(2)).sum::<f32>()
            / self.luminance_history.len() as f32;
        variance
    }
}

impl Default for AdaptationPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ctx(lux: f32, confidence: f32, context: &str) -> PolicyContext {
        PolicyContext {
            current_lux: lux,
            current_luminance: 50.0,
            app_context: context.into(),
            confidence,
            manual_override_active: false,
            is_fullscreen: false,
        }
    }

    #[test]
    fn test_manual_override_skips() {
        let mut policy = AdaptationPolicy::new();
        let mut ctx = make_ctx(200.0, 0.9, "Coding");
        ctx.manual_override_active = true;
        // Fill history
        for _ in 0..5 { policy.observe(200.0, 50.0); }
        let decision = policy.should_adapt(&ctx);
        assert!(!decision.is_adapt());
    }

    #[test]
    fn test_low_confidence_skips() {
        let mut policy = AdaptationPolicy::new();
        for _ in 0..5 { policy.observe(200.0, 50.0); }
        let ctx = make_ctx(200.0, 0.05, "Coding");
        assert!(!policy.should_adapt(&ctx).is_adapt());
    }

    #[test]
    fn test_fullscreen_gaming_skips() {
        let mut policy = AdaptationPolicy::new();
        for _ in 0..5 { policy.observe(200.0, 50.0); }
        let mut ctx = make_ctx(200.0, 0.9, "Gaming");
        ctx.is_fullscreen = true;
        assert!(!policy.should_adapt(&ctx).is_adapt());
    }

    #[test]
    fn test_insufficient_history_skips() {
        let mut policy = AdaptationPolicy::new();
        let ctx = make_ctx(200.0, 0.9, "Coding");
        // Only 1 observation — not enough
        policy.observe(200.0, 50.0);
        assert!(!policy.should_adapt(&ctx).is_adapt());
    }
}
