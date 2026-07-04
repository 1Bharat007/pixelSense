use crate::background::event::models::{AdaptiveEvent, AdaptiveEventKind, EventPriority};
use crate::performance::manager::PerformanceManager;
use std::sync::Arc;

/// Controls the sleep interval between pipeline cycles using the PerformanceManager.
///
/// ## Adaptive Scheduling
/// Defers interval logic to the Performance Optimization Engine, which calculates
/// the sleep time based on power states, full-screen apps, and static screen durations.
///
/// On a `Critical` event (wake, display removed), the interval is forced to
/// 100ms for the next 3 cycles, then normal backoff resumes.
pub struct PollingScheduler {
    performance_manager: Arc<PerformanceManager>,
    /// Force minimum interval for N more cycles (set after Critical events).
    force_minimum_cycles: u32,
}

impl PollingScheduler {
    pub fn new(performance_manager: Arc<PerformanceManager>) -> Self {
        Self {
            performance_manager,
            force_minimum_cycles: 0,
        }
    }

    /// Record that a cycle found no change. 
    pub fn on_no_change(&mut self) {
        self.performance_manager.report_screen_changed(false);
    }

    /// Record that a change was detected or a meaningful event was processed.
    pub fn on_change_detected(&mut self) {
        self.performance_manager.report_screen_changed(true);
    }

    /// Record that a Critical event was processed. Forces minimum interval for 3 cycles.
    pub fn on_critical_event(&mut self) {
        self.force_minimum_cycles = 3;
        self.performance_manager.report_screen_changed(true);
    }

    /// Returns the current sleep duration in milliseconds and advances the internal counter.
    pub fn next_interval_ms(&mut self) -> u64 {
        if self.force_minimum_cycles > 0 {
            self.force_minimum_cycles -= 1;
            return 100; // minimum emergency interval
        }
        
        let state = self.performance_manager.evaluate_performance_state();
        
        // Return the shorter interval between screen and ambient, 
        // since we need to wake up for whichever is faster.
        // If screen is paused, ambient interval dictates loop.
        if state.active_policy.pause_screen_analysis {
            state.active_policy.ambient_interval_ms
        } else {
            state.active_policy.screen_analysis_interval_ms.min(state.active_policy.ambient_interval_ms)
        }
    }

    /// Current interval exposed for diagnostics.
    pub fn current_interval_ms(&self) -> u64 {
        let state = self.performance_manager.evaluate_performance_state();
        if state.active_policy.pause_screen_analysis {
            state.active_policy.ambient_interval_ms
        } else {
            state.active_policy.screen_analysis_interval_ms.min(state.active_policy.ambient_interval_ms)
        }
    }

    /// Generate the periodic tick event for the event queue.
    pub fn make_tick_event(&self) -> AdaptiveEvent {
        AdaptiveEvent::new(AdaptiveEventKind::PeriodicTick, EventPriority::Low)
    }
}
