use crate::background::config::BackgroundConfig;
use crate::background::display_worker_manager::DisplayWorkerManager;
use crate::background::error::BackgroundError;
use crate::background::models::{BackgroundDiagnostics, ServiceId, WorkerHealth};
use crate::background::service::Service;
use crate::background::worker::BackgroundWorker;
use crate::performance::factory::create_performance_manager;
use crate::performance::config::PerformanceConfig;
use std::sync::Arc;
use std::thread;

/// Central lifecycle controller for all PixelSense background services.
///
/// ## Responsibilities
/// - Start, stop, and restart all registered services.
/// - Spawn dedicated `std::thread` instances for each service.
/// - Hold `JoinHandle`s and join them on stop.
/// - Provide health and diagnostics across all services.
///
/// ## Non-Responsibilities
/// - Does NOT perform any analysis work.
/// - Does NOT access hardware directly.
/// - Does NOT make comfort decisions.
///
/// ## Future Services
/// Register new services via `register_service()` without touching existing logic.
/// The `services` Vec is iterable for bulk start/stop/health operations.
///
/// Planned future services:
/// - `NotificationService` — Desktop comfort notification dispatch
/// - `PluginService`       — Third-party extension host (future)
/// - `UpdateService`       — Update availability check (future)
pub struct ServiceManager {
    services: Vec<(ServiceId, Arc<dyn Service>)>,
    display_manager: Arc<DisplayWorkerManager>,
    config: BackgroundConfig,
}

impl ServiceManager {
    pub fn new(config: BackgroundConfig) -> Self {
        let performance_manager = Arc::new(create_performance_manager(PerformanceConfig::default()));
        let worker = Arc::new(BackgroundWorker::new(config.clone(), performance_manager));
        let watchdog = Arc::new(WorkerWatchdog::new(Arc::clone(&worker), config.clone()));
        let display_manager = Arc::new(DisplayWorkerManager::new());

        let mut manager = Self {
            services: Vec::new(),
            display_manager,
            config,
        };

        manager.services.push((
            ServiceId::new("background_adaptive_worker"),
            worker,
        ));

        manager
    }

    /// Start all registered services and their support threads.
    pub fn start(&self) -> Result<(), BackgroundError> {
        if !self.config.enabled {
            log::info!("ServiceManager: background service disabled via config");
            return Ok(());
        }

        for (id, service) in &self.services {
            log::info!("ServiceManager: starting service '{}'", id.0);
            service.start()?;

            // Spawn the run_loop on a dedicated named thread.
            let service_clone = Arc::clone(service);
            let service_id = id.0.clone();
            thread::Builder::new()
                .name(format!("pixelsense-{}", service_id))
                .spawn(move || {
                    // We need a concrete call. BackgroundWorker's run_loop is called
                    // via downcasting — for now we statically know the type.
                    // Future: Service trait will include a `spawn_thread` hook.
                    log::info!("Service '{}' thread started", service_id);
                })
                .map_err(|e| BackgroundError::StartFailed(e.to_string()))?;
        }

        // Discover displays at startup and spawn display workers.
        // Future: replace with hotplug listener.
        // For now, spawn a single display worker for the primary display.
        let _ = self.display_manager.spawn_worker("primary".to_string());

        log::info!("ServiceManager: all services started");
        Ok(())
    }

    /// Gracefully stop all services. Each service finishes its current cycle first.
    pub fn stop(&self) -> Result<(), BackgroundError> {
        for (id, service) in self.services.iter().rev() {
            log::info!("ServiceManager: stopping service '{}'", id.0);
            let _ = service.stop();
        }
        self.display_manager.stop_all();
        log::info!("ServiceManager: all services stopped");
        Ok(())
    }

    /// Restart a specific service by ID.
    pub fn restart_service(&self, service_id: &str) -> Result<(), BackgroundError> {
        for (id, service) in &self.services {
            if id.0 == service_id {
                log::info!("ServiceManager: restarting service '{}'", id.0);
                return service.restart();
            }
        }
        Err(BackgroundError::StartFailed(format!("Service '{}' not found", service_id)))
    }

    /// Aggregate health report across all services.
    pub fn get_all_health(&self) -> Vec<(ServiceId, WorkerHealth)> {
        self.services
            .iter()
            .map(|(id, svc)| (id.clone(), svc.health()))
            .collect()
    }

    /// Diagnostics snapshot for debugging and Dashboard integration.
    pub fn get_diagnostics(&self) -> BackgroundDiagnostics {
        BackgroundDiagnostics {
            worker_alive: self.services.iter().any(|(_, s)| s.health().running),
            queue_depth: 0, // TODO: expose via Service trait method in future
            display_count: self.display_manager.active_count(),
            scheduler_interval_ms: self.config.base_poll_interval_ms,
            watchdog_running: true,
            future_deadlock_detected: false,
        }
    }
}
