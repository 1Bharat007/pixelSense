use crate::screen_analysis::models::FrameMetrics;
use std::collections::VecDeque;

/// Sliding history of recent frame metrics to detect flashes or rapid scene changes.
pub struct FrameHistory {
    metrics: VecDeque<FrameMetrics>,
    capacity: usize,
}

impl FrameHistory {
    pub fn new(capacity: usize) -> Self {
        Self {
            metrics: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, metric: FrameMetrics) {
        if self.metrics.len() >= self.capacity {
            self.metrics.pop_front();
        }
        self.metrics.push_back(metric);
    }

    /// Detects a sudden spike in average luminance (flash).
    /// Returns true if a flash is detected.
    pub fn detect_flash(&self, current: &FrameMetrics) -> bool {
        if let Some(last) = self.metrics.back() {
            // A rapid increase of more than 40% luminance between consecutive frames.
            if current.average_luminance > last.average_luminance + 40.0 {
                return true;
            }
        }
        false
    }
}
