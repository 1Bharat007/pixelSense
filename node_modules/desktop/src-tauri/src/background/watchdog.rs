use crate::background::config::BackgroundConfig;
use crate::background::error::BackgroundError;
use crate::background::models::{now_ms, WorkerHealth, WorkerId};
use crate::background::service::Service;
use crate::background::worker::BackgroundWorker;
use std::sync::Arc;
use std::thread;

/// Monitors the BackgroundWorker for frozen or crashed states and restarts it.
///
/// ## Detection
/// The Watchdog checks `WorkerHealth.last_heartbeat_ms` every `watchdog_timeout_ms / 2`.
/// If `now - last_heartbeat_ms > watchdog_timeout_ms`, the worker is considered frozen.
///
/// ## Restart
/// On detection, calls `ServiceManager::restart()` via the shared `worker` reference.
/// Increments `restart_count` in `WorkerHealth`.
///
/// ## Escalation
/// If `restart_count >= max_worker_restarts`, the Watchdog stops restarting.
/// `WorkerHealth.running` is set to false. The Dashboard will surface this state.
///
/// ## Non-Responsibilities
/// - Does NOT perform any analysis work.
/// - Does NOT interact with hardware.
/// - Does NOT communicate with the UI.
pub struct WorkerWatchdog {
    worker: Arc<BackgroundWorker>,
    config: BackgroundConfig,
    running: std::sync::atomic::AtomicBool,
}

impl WorkerWatchdog {
    pub fn new(worker: Arc<BackgroundWorker>, config: BackgroundConfig) -> Self {
        Self {
            worker,
            config,
            running: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Run the watchdog loop on a dedicated thread.
    pub fn run_loop(&self) {
        use std::sync::atomic::Ordering;
        self.running.store(true, Ordering::Relaxed);

        let check_interval = std::time::Duration::from_millis(self.config.watchdog_timeout_ms / 2);

        loop {
            thread::sleep(check_interval);

            if !self.running.load(Ordering::Relaxed) {
                break;
            }

            let health = self.worker.get_health();

            // Skip if not expected to be running
            match health.current_state {
                crate::background::models::WorkerState::Running
                | crate::background::models::WorkerState::Recovering => {}
                _ => continue,
            }

            let elapsed = now_ms().saturating_sub(health.last_heartbeat_ms);

            if elapsed > self.config.watchdog_timeout_ms {
                log::warn!(
                    "Watchdog: worker '{}' heartbeat stale by {}ms. Restarting.",
                    health.worker_id.0,
                    elapsed
                );

                if health.restart_count >= self.config.max_worker_restarts {
                    log::error!(
                        "Watchdog: worker '{}' exceeded max restarts ({}). Giving up.",
                        health.worker_id.0,
                        self.config.max_worker_restarts
                    );
                    // Signal the worker as permanently stopped — Watchdog cannot help further.
                    let _ = self.worker.stop();
                    break;
                }

                // Attempt restart
                match self.worker.restart() {
                    Ok(_) => {
                        log::info!("Watchdog: worker '{}' restarted successfully", health.worker_id.0);
                    }
                    Err(e) => {
                        log::error!("Watchdog: restart failed — {}", e);
                    }
                }
            }
        }

        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
        log::info!("WorkerWatchdog stopped");
    }

    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }
}
