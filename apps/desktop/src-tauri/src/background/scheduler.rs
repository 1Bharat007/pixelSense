use crate::background::config::BackgroundConfig;
use crate::background::event::models::{AdaptiveEvent, AdaptiveEventKind, EventPriority};
use crate::background::models::now_ms;

/// Controls the sleep interval between pipeline cycles.
///
/// ## Adaptive Backoff
/// When consecutive cycles detect no change, the interval increases by 25%
/// up to `maximum_poll_interval_ms`. This saves CPU when the screen is static
/// (e.g., user is reading a document with no content changes).
///
/// When a change is detected or a non-Low event arrives, the interval resets
/// to `base_poll_interval_ms`.
///
/// On a `Critical` event (wake, display removed), the interval is forced to
/// `minimum_poll_interval_ms` for the next 3 cycles, then normal backoff resumes.
pub struct PollingScheduler {
    config: BackgroundConfig,
    current_interval_ms: u64,
    /// How many consecutive cycles showed no change.
    no_change_streak: u32,
    /// Force minimum interval for N more cycles (set after Critical events).
    force_minimum_cycles: u32,
}

impl PollingScheduler {
    pub fn new(config: BackgroundConfig) -> Self {
        let base = config.base_poll_interval_ms;
        Self {
            config,
            current_interval_ms: base,
            no_change_streak: 0,
            force_minimum_cycles: 0,
        }
    }

    /// Record that a cycle found no change. Increases backoff if adaptive scheduling is enabled.
    pub fn on_no_change(&mut self) {
        self.no_change_streak += 1;

        if self.config.adaptive_scheduling_enabled && self.force_minimum_cycles == 0 {
            let next = (self.current_interval_ms as f64 * 1.25) as u64;
            self.current_interval_ms = next.min(self.config.maximum_poll_interval_ms);
        }
    }

    /// Record that a change was detected or a meaningful event was processed.
    pub fn on_change_detected(&mut self) {
        self.no_change_streak = 0;
        if self.force_minimum_cycles == 0 {
            self.current_interval_ms = self.config.base_poll_interval_ms;
        }
    }

    /// Record that a Critical event was processed. Forces minimum interval for 3 cycles.
    pub fn on_critical_event(&mut self) {
        self.force_minimum_cycles = 3;
        self.current_interval_ms = self.config.minimum_poll_interval_ms;
        self.no_change_streak = 0;
    }

    /// Returns the current sleep duration in milliseconds and advances the internal counter.
    pub fn next_interval_ms(&mut self) -> u64 {
        if self.force_minimum_cycles > 0 {
            self.force_minimum_cycles -= 1;
            if self.force_minimum_cycles == 0 {
                self.current_interval_ms = self.config.base_poll_interval_ms;
            }
            return self.config.minimum_poll_interval_ms;
        }
        self.current_interval_ms
    }

    /// Current interval exposed for diagnostics.
    pub fn current_interval_ms(&self) -> u64 {
        self.current_interval_ms
    }

    /// Generate the periodic tick event for the event queue.
    pub fn make_tick_event(&self) -> AdaptiveEvent {
        AdaptiveEvent::new(AdaptiveEventKind::PeriodicTick, EventPriority::Low)
    }
}
