use crate::background::error::BackgroundError;
use crate::background::models::{DisplayWorkerId, now_ms};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// Handle to a running per-display analysis thread.
pub struct DisplayWorkerHandle {
    pub id: DisplayWorkerId,
    pub display_id: String,
    pub cancel_token: Arc<AtomicBool>,
    pub started_at: u64,
}

/// Manages per-display background analysis threads.
///
/// ## Responsibilities
/// - Discover all active displays at startup and spawn one worker thread per display.
/// - Provide handles for stopping and restarting individual display workers.
/// - Report how many display workers are currently active.
///
/// ## Non-Responsibilities
/// - Does NOT perform analysis itself.
/// - Does NOT communicate with BrightnessEngine or TransitionEngine.
/// - Does NOT detect hotplug events (startup discovery only in this sprint).
///
/// ## Hotplug Future
/// Full hotplug support (via `WM_DEVICECHANGE` on Windows) is deferred to a future sprint.
/// When implemented, `DisplayWorkerManager` will own a hidden message-window thread
/// and call `add_display` / `remove_display` reactively.
pub struct DisplayWorkerManager {
    workers: Mutex<HashMap<String, DisplayWorkerHandle>>,
}

impl DisplayWorkerManager {
    pub fn new() -> Self {
        Self {
            workers: Mutex::new(HashMap::new()),
        }
    }

    /// Spawn a lightweight monitoring thread for the given display.
    /// The thread runs until its `cancel_token` is set to true.
    pub fn spawn_worker(
        &self,
        display_id: String,
    ) -> Result<(), BackgroundError> {
        let cancel_token = Arc::new(AtomicBool::new(false));
        let token_clone = Arc::clone(&cancel_token);
        let display_id_clone = display_id.clone();

        thread::Builder::new()
            .name(format!("pixelsense-display-{}", display_id))
            .spawn(move || {
                // This thread's loop will be expanded when full per-display
                // screen analysis is wired up from ScreenAnalysisManager.
                while !token_clone.load(Ordering::Relaxed) {
                    // Placeholder: real work dispatched by BackgroundWorker
                    thread::sleep(std::time::Duration::from_millis(500));
                }
                log::info!("Display worker for {} stopped", display_id_clone);
            })
            .map_err(|e| BackgroundError::DisplayWorkerFailed(e.to_string()))?;

        let handle = DisplayWorkerHandle {
            id: DisplayWorkerId::new(&display_id),
            display_id: display_id.clone(),
            cancel_token,
            started_at: now_ms(),
        };

        if let Ok(mut workers) = self.workers.lock() {
            workers.insert(display_id, handle);
        }

        Ok(())
    }

    /// Signal a specific display worker to stop.
    pub fn remove_display(&self, display_id: &str) {
        if let Ok(mut workers) = self.workers.lock() {
            if let Some(handle) = workers.remove(display_id) {
                handle.cancel_token.store(true, Ordering::Relaxed);
                log::info!("Display worker for {} signalled to stop", display_id);
            }
        }
    }

    /// Signal all display workers to stop.
    pub fn stop_all(&self) {
        if let Ok(mut workers) = self.workers.lock() {
            for (_, handle) in workers.drain() {
                handle.cancel_token.store(true, Ordering::Relaxed);
            }
        }
    }

    /// Current number of active display workers.
    pub fn active_count(&self) -> usize {
        self.workers.lock().map(|w| w.len()).unwrap_or(0)
    }

    /// Returns all currently managed display IDs.
    pub fn display_ids(&self) -> Vec<String> {
        self.workers
            .lock()
            .map(|w| w.keys().cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for DisplayWorkerManager {
    fn default() -> Self {
        Self::new()
    }
}
