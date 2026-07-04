use crate::background::error::BackgroundError;
use crate::background::models::WorkerHealth;

/// Abstract contract for all background services managed by `ServiceManager`.
///
/// ## Registered Services (Current & Future)
/// | Service            | Status      | Purpose                            |
/// |--------------------|-------------|------------------------------------|
/// | BackgroundWorker   | Sprint 16   | Adaptive comfort pipeline          |
/// | NotificationService| Planned     | Desktop notification dispatch      |
/// | PluginService      | Future      | Third-party extension host         |
/// | UpdateService      | Future      | Update availability check          |
///
/// ## Non-Responsibilities
/// Each `Service` implementation must NOT manage other services.
/// Lifecycle orchestration is exclusively `ServiceManager`'s domain.
pub trait Service: Send + Sync {
    /// Returns the unique identifier for this service.
    fn service_id(&self) -> &str;

    /// Start the service. Must be idempotent if already running (return `AlreadyRunning`).
    fn start(&self) -> Result<(), BackgroundError>;

    /// Stop the service gracefully. Must wait for in-progress work to complete.
    /// Must NOT terminate during a hardware write.
    fn stop(&self) -> Result<(), BackgroundError>;

    /// Stop and restart the service. Used by `ServiceManager` and `WorkerWatchdog`.
    fn restart(&self) -> Result<(), BackgroundError> {
        self.stop()?;
        self.start()
    }

    /// Returns the latest health snapshot for this service.
    fn health(&self) -> WorkerHealth;
}
