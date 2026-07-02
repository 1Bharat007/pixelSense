#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundError {
    /// Worker failed to start (e.g., thread spawn failed).
    StartFailed(String),
    /// Worker failed to stop within the timeout. Threads may be detached.
    StopTimeout(String),
    /// A specific display worker could not be spawned.
    DisplayWorkerFailed(String),
    /// The service was requested to start but is already running.
    AlreadyRunning,
    /// The service was requested to stop but is already stopped.
    AlreadyStopped,
    /// The Watchdog detected a frozen worker and could not restart it.
    WatchdogRestartFailed(String),
    /// Maximum restart attempts exceeded. Service is permanently degraded.
    MaxRestartsExceeded,
}

impl std::fmt::Display for BackgroundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartFailed(m) => write!(f, "Start failed: {}", m),
            Self::StopTimeout(m) => write!(f, "Stop timed out: {}", m),
            Self::DisplayWorkerFailed(m) => write!(f, "Display worker failed: {}", m),
            Self::AlreadyRunning => write!(f, "Service is already running"),
            Self::AlreadyStopped => write!(f, "Service is already stopped"),
            Self::WatchdogRestartFailed(m) => write!(f, "Watchdog restart failed: {}", m),
            Self::MaxRestartsExceeded => write!(f, "Maximum worker restarts exceeded"),
        }
    }
}

impl std::error::Error for BackgroundError {}
