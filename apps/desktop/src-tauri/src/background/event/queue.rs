use crate::background::event::models::{AdaptiveEvent, AdaptiveEventKind, EventPriority};
use std::collections::VecDeque;
use std::sync::Mutex;

/// The maximum number of non-Critical events held in the queue.
/// When full, incoming `Low` priority events are dropped first.
const MAX_QUEUE_DEPTH: usize = 64;

/// Priority-ordered, deduplication-aware event queue.
///
/// ## Ordering
/// Events are returned in priority order (Critical first), then FIFO within the same priority.
///
/// ## Deduplication
/// If a new event arrives with the same `AdaptiveEventKind` as the most recent event
/// of the same priority already in the queue, the new event is merged (ignored).
/// `Critical` events are **never** deduplicated.
///
/// ## Backpressure
/// If the queue exceeds `MAX_QUEUE_DEPTH`, incoming `Low` priority events are dropped.
/// Higher priority events always succeed.
pub struct EventQueue {
    // Separate queues per priority for O(1) push and deduplication.
    critical: Mutex<VecDeque<AdaptiveEvent>>,
    high: Mutex<VecDeque<AdaptiveEvent>>,
    normal: Mutex<VecDeque<AdaptiveEvent>>,
    low: Mutex<VecDeque<AdaptiveEvent>>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            critical: Mutex::new(VecDeque::new()),
            high: Mutex::new(VecDeque::new()),
            normal: Mutex::new(VecDeque::new()),
            low: Mutex::new(VecDeque::new()),
        }
    }

    /// Push an event. Applies deduplication and backpressure.
    pub fn push(&self, event: AdaptiveEvent) {
        match event.priority {
            EventPriority::Critical => {
                // Critical: never deduplicate, never drop.
                if let Ok(mut q) = self.critical.lock() {
                    q.push_back(event);
                }
            }
            EventPriority::High => {
                if let Ok(mut q) = self.high.lock() {
                    if !Self::is_duplicate(&q, &event.kind) {
                        q.push_back(event);
                    }
                }
            }
            EventPriority::Normal => {
                if let Ok(mut q) = self.normal.lock() {
                    if q.len() < MAX_QUEUE_DEPTH && !Self::is_duplicate(&q, &event.kind) {
                        q.push_back(event);
                    }
                }
            }
            EventPriority::Low => {
                if let Ok(mut q) = self.low.lock() {
                    // Strict: only one Low-priority event at a time.
                    q.clear();
                    q.push_back(event);
                }
            }
        }
    }

    /// Pop the highest-priority pending event. Returns None if empty.
    pub fn pop_next(&self) -> Option<AdaptiveEvent> {
        if let Ok(mut q) = self.critical.lock() {
            if let Some(e) = q.pop_front() {
                return Some(e);
            }
        }
        if let Ok(mut q) = self.high.lock() {
            if let Some(e) = q.pop_front() {
                return Some(e);
            }
        }
        if let Ok(mut q) = self.normal.lock() {
            if let Some(e) = q.pop_front() {
                return Some(e);
            }
        }
        if let Ok(mut q) = self.low.lock() {
            if let Some(e) = q.pop_front() {
                return Some(e);
            }
        }
        None
    }

    /// Drain all Critical events without touching other queues.
    pub fn drain_critical(&self) -> Vec<AdaptiveEvent> {
        self.critical.lock().map(|mut q| q.drain(..).collect()).unwrap_or_default()
    }

    /// Total events across all queues.
    pub fn len(&self) -> usize {
        let c = self.critical.lock().map(|q| q.len()).unwrap_or(0);
        let h = self.high.lock().map(|q| q.len()).unwrap_or(0);
        let n = self.normal.lock().map(|q| q.len()).unwrap_or(0);
        let l = self.low.lock().map(|q| q.len()).unwrap_or(0);
        c + h + n + l
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if an event of the same kind already exists in the queue.
    fn is_duplicate(queue: &VecDeque<AdaptiveEvent>, kind: &AdaptiveEventKind) -> bool {
        queue.iter().any(|e| &e.kind == kind)
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}
