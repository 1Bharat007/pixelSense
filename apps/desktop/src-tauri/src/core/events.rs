use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPayload {
    pub version: u32,
    pub data: String, // JSON payload
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppEvent {
    pub event_id: String,
    pub timestamp: u64,
    pub correlation_id: Option<String>,
    pub source: String,
    pub priority: u8,
    pub payload: EventPayload,
}

pub trait EventSubscriber: Send + Sync {
    fn on_event(&self, event: &AppEvent);
}

// In a full implementation, a global EventBus would manage dispatching to subscribers.
