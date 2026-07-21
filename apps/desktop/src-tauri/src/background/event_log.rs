use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

/// Maximum events stored in memory.
const MAX_EVENTS: usize = 25;

/// Categories of events PixelSense can log.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventCategory {
    ProtectionToggled,
    BrightnessChanged,
    ContextChanged,
    SensorEvent,
    AdaptationSkipped,
    SystemEvent,
}

impl EventCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ProtectionToggled => "Protection",
            Self::BrightnessChanged => "Brightness",
            Self::ContextChanged => "Context",
            Self::SensorEvent => "Sensor",
            Self::AdaptationSkipped => "Skipped",
            Self::SystemEvent => "System",
        }
    }
}

/// A single recorded event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: u64,
    /// Category for filtering/display.
    pub category: EventCategory,
    /// Human-readable description.
    pub description: String,
    /// Optional: value before the change (e.g. "72%").
    pub before_value: Option<String>,
    /// Optional: value after the change (e.g. "65%").
    pub after_value: Option<String>,
}

impl LogEvent {
    pub fn new(category: EventCategory, description: impl Into<String>) -> Self {
        Self {
            timestamp_ms: now_ms(),
            category,
            description: description.into(),
            before_value: None,
            after_value: None,
        }
    }

    pub fn with_values(mut self, before: impl Into<String>, after: impl Into<String>) -> Self {
        self.before_value = Some(before.into());
        self.after_value = Some(after.into());
        self
    }
}

/// EventLog is the lightweight in-memory event history for PixelSense.
/// It stores the last 25 events and exposes them to the Developer console.
///
/// Responsibility: Keep a running audit trail of what PixelSense decided and why.
/// This is the primary debugging tool for validating that decisions are correct.
pub struct EventLog {
    events: VecDeque<LogEvent>,
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            events: VecDeque::with_capacity(MAX_EVENTS),
        }
    }

    /// Add a new event. Oldest event is dropped when at capacity.
    pub fn push(&mut self, event: LogEvent) {
        if self.events.len() >= MAX_EVENTS {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    /// Returns events in reverse-chronological order (newest first).
    pub fn get_recent(&self) -> Vec<LogEvent> {
        self.events.iter().cloned().rev().collect()
    }

    /// Returns the total number of events stored.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared reference type for EventLog.
pub type SharedEventLog = Arc<Mutex<EventLog>>;

pub fn new_shared_event_log() -> SharedEventLog {
    Arc::new(Mutex::new(EventLog::new()))
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity_limit() {
        let mut log = EventLog::new();
        for i in 0..30 {
            log.push(LogEvent::new(EventCategory::SystemEvent, format!("Event {}", i)));
        }
        assert_eq!(log.len(), MAX_EVENTS);
    }

    #[test]
    fn test_newest_first_order() {
        let mut log = EventLog::new();
        log.push(LogEvent::new(EventCategory::BrightnessChanged, "First"));
        log.push(LogEvent::new(EventCategory::BrightnessChanged, "Second"));
        let recent = log.get_recent();
        assert_eq!(recent[0].description, "Second");
        assert_eq!(recent[1].description, "First");
    }

    #[test]
    fn test_with_values() {
        let event = LogEvent::new(EventCategory::BrightnessChanged, "Adjusted")
            .with_values("72%", "65%");
        assert_eq!(event.before_value, Some("72%".into()));
        assert_eq!(event.after_value, Some("65%".into()));
    }
}
