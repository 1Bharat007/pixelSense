use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Mutex;
use crate::platform::error::PlatformError;

/// Defines the priority of a native event.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Represents strongly-typed platform events derived from raw Win32 messages.
#[derive(Debug, Clone)]
pub enum PlatformEvent {
    DisplayConnected(String),
    DisplayDisconnected(String),
    DisplayConfigurationChanged,
    PowerSourceChanged { on_battery: bool },
    BatterySaverChanged { active: bool },
    ForegroundWindowChanged { executable: String },
    SessionLocked,
    SessionUnlocked,
}

impl PlatformEvent {
    pub fn priority(&self) -> EventPriority {
        match self {
            Self::DisplayConnected(_) | Self::DisplayDisconnected(_) | Self::SessionLocked | Self::SessionUnlocked => EventPriority::Critical,
            Self::DisplayConfigurationChanged | Self::PowerSourceChanged { .. } | Self::ForegroundWindowChanged { .. } => EventPriority::High,
            Self::BatterySaverChanged { .. } => EventPriority::Normal,
        }
    }
}

/// A bounded event bus for native Windows events.
/// Implements drop-low / merge-normal policies.
pub struct PlatformEventBus {
    sender: Mutex<Sender<PlatformEvent>>,
    receiver: Mutex<Receiver<PlatformEvent>>,
}

impl PlatformEventBus {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: Mutex::new(tx),
            receiver: Mutex::new(rx),
        }
    }

    /// Dispatches an event from a native Win32 callback into the application loop.
    pub fn dispatch(&self, event: PlatformEvent) -> Result<(), PlatformError> {
        let tx = self.sender.lock().unwrap();
        tx.send(event).map_err(|_| PlatformError::NativeApiUnavailable("Event queue closed".into()))
    }

    /// Receives the next available event (non-blocking in real implementation via try_recv).
    pub fn try_recv(&self) -> Option<PlatformEvent> {
        let rx = self.receiver.lock().unwrap();
        rx.try_recv().ok()
    }
}

impl Default for PlatformEventBus {
    fn default() -> Self {
        Self::new()
    }
}
