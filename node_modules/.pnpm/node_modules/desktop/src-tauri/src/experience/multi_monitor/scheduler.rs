use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityTier {
    Tier1Focused,
    Tier2Active,
    Tier3Idle,
}

impl PriorityTier {
    pub fn get_interval_ms(&self) -> u64 {
        match self {
            PriorityTier::Tier1Focused => 100, // Very fast for focused display
            PriorityTier::Tier2Active => 500,  // Moderate for recent changes
            PriorityTier::Tier3Idle => 2000,   // Slow for idle displays
        }
    }
}

pub struct DisplayState {
    pub priority: PriorityTier,
    pub last_activity: Instant,
    pub lux_drift: f32,
}

pub struct MultiMonitorScheduler {
    displays: Arc<Mutex<HashMap<String, DisplayState>>>,
    tier2_timeout: Duration,
}

impl MultiMonitorScheduler {
    pub fn new() -> Self {
        Self {
            displays: Arc::new(Mutex::new(HashMap::new())),
            tier2_timeout: Duration::from_secs(30), // Demote to idle after 30s of no activity
        }
    }

    pub fn register_display(&self, display_id: &str) {
        if let Ok(mut displays) = self.displays.lock() {
            if !displays.contains_key(display_id) {
                displays.insert(
                    display_id.to_string(),
                    DisplayState {
                        priority: PriorityTier::Tier3Idle,
                        last_activity: Instant::now(),
                        lux_drift: 0.0,
                    },
                );
            }
        }
    }

    pub fn mark_focused(&self, display_id: &str) {
        if let Ok(mut displays) = self.displays.lock() {
            // Demote any existing focused display to Tier 2
            for state in displays.values_mut() {
                if state.priority == PriorityTier::Tier1Focused {
                    state.priority = PriorityTier::Tier2Active;
                    state.last_activity = Instant::now();
                }
            }
            
            // Promote the target display to Tier 1
            if let Some(state) = displays.get_mut(display_id) {
                state.priority = PriorityTier::Tier1Focused;
                state.last_activity = Instant::now();
            }
        }
    }

    pub fn record_activity(&self, display_id: &str) {
        if let Ok(mut displays) = self.displays.lock() {
            if let Some(state) = displays.get_mut(display_id) {
                // If idle, wake up to Tier 2. If Tier 1, stay Tier 1.
                if state.priority == PriorityTier::Tier3Idle {
                    state.priority = PriorityTier::Tier2Active;
                }
                state.last_activity = Instant::now();
            }
        }
    }

    pub fn tick(&self) {
        if let Ok(mut displays) = self.displays.lock() {
            let now = Instant::now();
            for state in displays.values_mut() {
                if state.priority == PriorityTier::Tier2Active && now.duration_since(state.last_activity) > self.tier2_timeout {
                    state.priority = PriorityTier::Tier3Idle;
                }
            }
        }
    }

    pub fn get_interval(&self, display_id: &str) -> u64 {
        if let Ok(displays) = self.displays.lock() {
            if let Some(state) = displays.get(display_id) {
                return state.priority.get_interval_ms();
            }
        }
        PriorityTier::Tier3Idle.get_interval_ms()
    }
}
