use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", content = "data")]
pub enum HistoryEvent {
    BrightnessChanged {
        display_id: String,
        old_level: u8,
        new_level: u8,
        reason: String,
    },
    AmbientChanged {
        lux: f32,
        environment: String,
    },
    ComfortProfileMatched {
        profile_id: String,
    },
    TransitionStarted {
        target_brightness: u8,
        duration_ms: u64,
    },
    TransitionFinished {
        final_brightness: u8,
    },
    PowerModeChanged {
        new_state: String,
    },
    FullscreenEntered {
        app_name: String,
    },
    FullscreenExited {
        app_name: String,
    },
    ManualOverride {
        display_id: String,
        user_level: u8,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampedEvent {
    pub timestamp: u64, // Unix timestamp in milliseconds
    pub event: HistoryEvent,
}
