use crate::background::models::PipelineProfile;
use std::sync::Mutex;

/// Stores timing for the most recent pipeline cycle only.
///
/// ## Why No History?
/// Storing a rolling history would grow memory unboundedly over time.
/// The Dashboard will read the latest profile on demand. Historical graphs
/// are a future feature that will use a fixed-size ring buffer when needed.
pub struct PipelineProfiler {
    latest: Mutex<Option<PipelineProfile>>,
}

impl PipelineProfiler {
    pub fn new() -> Self {
        Self {
            latest: Mutex::new(None),
        }
    }

    /// Store the result of the latest cycle. Overwrites any previous value.
    pub fn record(&self, profile: PipelineProfile) {
        if let Ok(mut guard) = self.latest.lock() {
            *guard = Some(profile);
        }
    }

    /// Returns a clone of the latest profile, or `None` if no cycle has run yet.
    pub fn get_latest(&self) -> Option<PipelineProfile> {
        self.latest.lock().ok().and_then(|g| g.clone())
    }
}

impl Default for PipelineProfiler {
    fn default() -> Self {
        Self::new()
    }
}
