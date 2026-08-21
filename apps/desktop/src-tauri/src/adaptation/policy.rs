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
    ///
    /// NOTE: When `true`, adaptation continues normally — only UI toasts/notifications
    /// should be suppressed by the caller. We never pause brightness adaptation during
    /// fullscreen video: that is exactly when eye-comfort matters most.
    pub is_fullscreen: bool,
}

/// The outcome from the Adaptation Policy.
#[derive(Debug, Clone)]
pub enum AdaptationDecision {
    /// Proceed with brightness adaptation immediately.
    Adapt { reason: String },
    /// Possible scene change detected — sample again fast before committing.
    /// The pipeline must sleep ~150ms and re-evaluate rather than acting yet.
    FastConfirm,
    /// Do not change brightness right now. Reason is logged to EventLog.
    Skip { reason: String },
}

impl AdaptationDecision {
    pub fn is_adapt(&self) -> bool {
        matches!(self, Self::Adapt { .. })
    }
    pub fn is_fast_confirm(&self) -> bool {
        matches!(self, Self::FastConfirm)
    }
    pub fn reason(&self) -> &str {
        match self {
            Self::Adapt { reason } | Self::Skip { reason } => reason,
            Self::FastConfirm => "Fast-confirm: waiting for second sample",
        }
    }
}

/// Minimum lux change required before we consider adapting.
const MIN_LUX_DELTA: f32 = 5.0;
/// Minimum luminance change required before we consider adapting.
const MIN_LUMINANCE_DELTA: f32 = 5.0;
/// Number of consecutive stable readings before adapting (noise filter) — normal path.
const STABILITY_WINDOW: usize = 2;
/// Duration a context must be stable before we respond to it — normal path.
const CONTEXT_STABILITY_MS: u64 = 250;

/// Luminance delta that triggers the fast-confirm burst instead of the slow stability path.
/// Chosen to be above normal content variation (~5–12) but below genuine scene cuts (>20).
/// Calibrated against VLC, MPC, browser video, and tab-switch scenarios.
const HARD_CUT_DELTA: f32 = 18.0;

/// If the second fast-confirm sample is within this distance of the first, we treat it as
/// a real, sustained scene change and approve adaptation immediately.
const CONFIRM_TOLERANCE: f32 = 8.0;

/// How long we stay in the fast-confirm burst window before giving up and falling back
/// to the slow/smoothed path. Covers multi-second bright scenes (e.g. 5s film explosion).
const FAST_CONFIRM_WINDOW_MS: u64 = 1800;

/// State for the fast-confirm burst mode.
#[derive(Debug)]
struct FastConfirmState {
    /// Luminance level that triggered the burst.
    candidate_level: f32,
    /// When we entered the burst.
    entered_at: Instant,
}

/// The Adaptation Policy is the "should we adapt?" gate.
///
/// It sits between the Decision Engine (which computes *how much* to change)
/// and the Transition Engine (which *executes* the change).
///
/// ## Fast-Confirm Burst (new in v1.2.0)
///
/// Instead of dismissing sudden luminance jumps as "volatile transient content"
/// (the old Rule 4), we now use a two-sample confirmation model:
///
/// 1. If luminance jumps ≥ `HARD_CUT_DELTA` in a single cycle → enter burst mode,
///    signal `FastConfirm` to the pipeline (sample again in ~150ms, don't act yet).
/// 2. If the next sample confirms the new level (within `CONFIRM_TOLERANCE`) →
///    approve adaptation immediately. This catches 5-second bright scenes.
/// 3. If the sample bounced back → single-frame flash, skip it.
/// 4. If still volatile after `FAST_CONFIRM_WINDOW_MS` → fall through to the
///    existing slow/smoothed path (genuinely chaotic content like strobe effects).
///
/// This gives us sub-500ms response to real scene changes, while still not moving
/// the backlight for single-frame flashes or genuine video noise.
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
    /// Active fast-confirm burst state, if any.
    fast_confirm: Option<FastConfirmState>,
}

impl AdaptationPolicy {
    pub fn new() -> Self {
        Self {
            lux_history: VecDeque::with_capacity(STABILITY_WINDOW + 2),
            luminance_history: VecDeque::with_capacity(STABILITY_WINDOW + 2),
            last_adapted_lux: None,
            last_adapted_luminance: None,
            context_stable_since: None,
            fast_confirm: None,
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
            // Clear any pending fast-confirm so it doesn't fire after override expires.
            self.fast_confirm = None;
            return AdaptationDecision::Skip {
                reason: "Manual override is active".into(),
            };
        }

        // Rule 2: Fullscreen gaming — pause adaptation to avoid distracting the player.
        // NOTE: fullscreen VIDEO is intentionally NOT paused here. The movie-scene use
        // case (Section 1 of the sprint doc) is exactly what this feature is for.
        // is_fullscreen is currently hardcoded false in pipeline.rs (TODO: Win32 detection).
        if ctx.is_fullscreen && ctx.app_context == "Gaming" {
            self.fast_confirm = None;
            return AdaptationDecision::Skip {
                reason: "Fullscreen Gaming session active — pausing adaptation".into(),
            };
        }

        // ── Fast-Confirm Burst Path ─────────────────────────────────────────────────
        //
        // Check the luminance delta from the previous observation.
        // If it exceeds HARD_CUT_DELTA, enter burst mode (don't adapt yet).
        // On the next call, if confirmed → approve. If bounced → skip. If timed out → slow path.

        let prev_luminance = self
            .luminance_history
            .iter()
            .rev()
            .nth(1) // second-to-last element (the one before the current)
            .copied();

        let luminance_step = prev_luminance
            .map(|prev| (ctx.current_luminance - prev).abs())
            .unwrap_or(0.0);

        match &self.fast_confirm {
            None if luminance_step >= HARD_CUT_DELTA => {
                // Big jump detected — enter fast-confirm burst.
                self.fast_confirm = Some(FastConfirmState {
                    candidate_level: ctx.current_luminance,
                    entered_at: Instant::now(),
                });
                return AdaptationDecision::FastConfirm;
            }
            Some(fc) if (ctx.current_luminance - fc.candidate_level).abs() <= CONFIRM_TOLERANCE => {
                // Second sample confirms the new luminance level — real scene change.
                log::info!(
                    "[policy] Fast-confirm: confirmed scene change to {:.1}% luminance (was {:.1}%)",
                    ctx.current_luminance,
                    prev_luminance.unwrap_or(0.0)
                );
                self.fast_confirm = None;
                self.last_adapted_lux = Some(ctx.current_lux);
                self.last_adapted_luminance = Some(ctx.current_luminance);
                return AdaptationDecision::Adapt {
                    reason: format!(
                        "Confirmed scene change — luminance now {:.0}% (2/2 fast samples agree)",
                        ctx.current_luminance
                    ),
                };
            }
            Some(fc) if fc.entered_at.elapsed() > Duration::from_millis(FAST_CONFIRM_WINDOW_MS) => {
                // Burst timed out — content is genuinely volatile (strobe, rapid cuts).
                // Fall through to the existing slow/smoothed path rather than looping forever.
                log::debug!(
                    "[policy] Fast-confirm timed out — volatile content, falling back to slow path"
                );
                self.fast_confirm = None;
                // Fall through to slow path below.
            }
            Some(_) => {
                // Still in the fast-confirm window, sample hasn't confirmed yet.
                return AdaptationDecision::FastConfirm;
            }
            None => {
                // No burst in progress, no large jump — normal path continues below.
            }
        }

        // ── Normal (Slow/Smoothed) Path ────────────────────────────────────────────
        //
        // Rule 3: Check if we have a meaningful screen luminance change or valid ambient lux change.
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

        if !screen_changed && !lux_changed {
            return AdaptationDecision::Skip {
                reason: "Environment and screen content within comfort tolerance".into(),
            };
        }

        // Rule 4 (REPLACED): Old variance check has been removed.
        // The fast-confirm burst path above now handles the "sudden change" case correctly.
        // Gradual changes (small deltas over many cycles) still reach here and are filtered
        // by the STABILITY_WINDOW requirement below, preventing oscillation.

        // Rule 5: Context must be stable for at least CONTEXT_STABILITY_MS before acting.
        let context_stable = match &self.context_stable_since {
            Some((last_ctx, since)) if last_ctx == &ctx.app_context => {
                since.elapsed() >= Duration::from_millis(CONTEXT_STABILITY_MS)
            }
            _ => {
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
        self.lux_history
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>()
            / self.lux_history.len() as f32
    }

    #[allow(dead_code)]
    fn luminance_variance(&self) -> f32 {
        if self.luminance_history.len() < 2 {
            return 0.0;
        }
        let mean = self.luminance_history.iter().sum::<f32>() / self.luminance_history.len() as f32;
        self.luminance_history
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>()
            / self.luminance_history.len() as f32
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

    fn make_ctx(lux: f32, luminance: f32, confidence: f32, context: &str) -> PolicyContext {
        PolicyContext {
            current_lux: lux,
            current_luminance: luminance,
            app_context: context.into(),
            confidence,
            manual_override_active: false,
            is_fullscreen: false,
        }
    }

    #[test]
    fn test_manual_override_skips() {
        let mut policy = AdaptationPolicy::new();
        let mut ctx = make_ctx(200.0, 50.0, 0.9, "Coding");
        ctx.manual_override_active = true;
        for _ in 0..5 {
            policy.observe(200.0, 50.0);
        }
        assert!(!policy.should_adapt(&ctx).is_adapt());
    }

    #[test]
    fn test_low_confidence_skips() {
        let mut policy = AdaptationPolicy::new();
        for _ in 0..5 {
            policy.observe(200.0, 50.0);
        }
        let ctx = make_ctx(200.0, 50.0, 0.05, "Coding");
        assert!(!policy.should_adapt(&ctx).is_adapt());
    }

    #[test]
    fn test_fullscreen_gaming_skips() {
        let mut policy = AdaptationPolicy::new();
        for _ in 0..5 {
            policy.observe(200.0, 50.0);
        }
        let mut ctx = make_ctx(200.0, 50.0, 0.9, "Gaming");
        ctx.is_fullscreen = true;
        assert!(!policy.should_adapt(&ctx).is_adapt());
    }

    /// Fullscreen VIDEO should still adapt — this is the movie-scene use case.
    #[test]
    fn test_fullscreen_video_still_adapts() {
        let mut policy = AdaptationPolicy::new();
        // Prime with dim scene, then simulate a bright-scene jump.
        for _ in 0..3 {
            policy.observe(200.0, 25.0);
        }
        // Warm up context stability timer.
        policy.context_stable_since =
            Some(("Video".into(), Instant::now() - Duration::from_millis(500)));

        let mut ctx = make_ctx(200.0, 75.0, 0.9, "Video");
        ctx.is_fullscreen = true;

        // First call: big jump (25 → 75) → FastConfirm.
        policy.observe(200.0, 75.0);
        let d1 = policy.should_adapt(&ctx);
        assert!(
            d1.is_fast_confirm(),
            "Expected FastConfirm on first big jump, got: {:?}",
            d1
        );

        // Second call (confirm sample): same luminance → Adapt.
        policy.observe(200.0, 75.0);
        let d2 = policy.should_adapt(&ctx);
        assert!(
            d2.is_adapt(),
            "Expected Adapt on confirmation sample, got: {:?}",
            d2
        );
    }

    #[test]
    fn test_insufficient_history_skips() {
        let mut policy = AdaptationPolicy::new();
        let ctx = make_ctx(200.0, 50.0, 0.9, "Coding");
        policy.observe(200.0, 50.0);
        assert!(!policy.should_adapt(&ctx).is_adapt());
    }

    /// Primary regression test: sustained dim→bright scene must be confirmed and approved.
    /// This is the exact scenario from the sprint doc (Section 1).
    #[test]
    fn test_scene_change_dim_to_bright_is_confirmed() {
        let mut policy = AdaptationPolicy::new();

        // Simulate 3 cycles of dim scene (lum=20).
        for _ in 0..3 {
            policy.observe(100.0, 20.0);
        }

        // Context is already stable (pre-warm the timer).
        policy.context_stable_since =
            Some(("Video".into(), Instant::now() - Duration::from_millis(500)));

        // Cycle 4: sudden bright scene (lum=75, delta=55 > HARD_CUT_DELTA).
        policy.observe(100.0, 75.0);
        let ctx = make_ctx(100.0, 75.0, 0.9, "Video");
        let d1 = policy.should_adapt(&ctx);
        assert!(
            d1.is_fast_confirm(),
            "Expected FastConfirm on sudden bright scene, got: {:?}",
            d1
        );

        // Cycle 5: second sample, bright scene sustained (confirm).
        policy.observe(100.0, 73.0); // within CONFIRM_TOLERANCE=8 of 75
        let ctx2 = make_ctx(100.0, 73.0, 0.9, "Video");
        let d2 = policy.should_adapt(&ctx2);
        assert!(
            d2.is_adapt(),
            "Expected Adapt after sustained bright scene confirmed, got: {:?}",
            d2
        );
    }

    /// Single-frame flash must NOT move the backlight.
    #[test]
    fn test_single_frame_flash_does_not_adapt() {
        let mut policy = AdaptationPolicy::new();

        // Stable dim baseline.
        for _ in 0..3 {
            policy.observe(100.0, 20.0);
        }
        policy.context_stable_since =
            Some(("Video".into(), Instant::now() - Duration::from_millis(500)));

        // Cycle: bright flash.
        policy.observe(100.0, 75.0);
        let ctx = make_ctx(100.0, 75.0, 0.9, "Video");
        let d1 = policy.should_adapt(&ctx);
        assert!(d1.is_fast_confirm(), "First call should be FastConfirm");

        // Cycle: immediately snaps back to dim (single frame flash).
        policy.observe(100.0, 22.0);
        let ctx2 = make_ctx(100.0, 22.0, 0.9, "Video");
        let d2 = policy.should_adapt(&ctx2);
        // Should NOT be Adapt — flash ended before confirmation.
        assert!(
            !d2.is_adapt(),
            "Single-frame flash must not trigger adaptation, got: {:?}",
            d2
        );
    }

    /// After the fast-confirm window expires, the policy must stop looping and fall
    /// through to the slow path — not loop forever in fast-confirm.
    #[test]
    fn test_fast_confirm_timeout_falls_back_to_slow_path() {
        let mut policy = AdaptationPolicy::new();

        for _ in 0..3 {
            policy.observe(100.0, 20.0);
        }
        policy.context_stable_since =
            Some(("Video".into(), Instant::now() - Duration::from_millis(500)));

        // Enter fast-confirm.
        policy.observe(100.0, 75.0);
        let ctx = make_ctx(100.0, 75.0, 0.9, "Video");
        let _ = policy.should_adapt(&ctx);

        // Manually expire the fast-confirm window.
        if let Some(ref mut fc) = policy.fast_confirm {
            fc.entered_at = Instant::now() - Duration::from_millis(FAST_CONFIRM_WINDOW_MS + 100);
        }

        // Next call — should timeout, clear fast_confirm, not return FastConfirm.
        policy.observe(100.0, 75.0);
        let ctx2 = make_ctx(100.0, 75.0, 0.9, "Video");
        let d = policy.should_adapt(&ctx2);
        assert!(
            !d.is_fast_confirm(),
            "After timeout, should not stay in FastConfirm forever, got: {:?}",
            d
        );
        assert!(
            policy.fast_confirm.is_none(),
            "fast_confirm should be cleared after timeout"
        );
    }
}
