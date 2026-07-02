use std::time::{SystemTime, UNIX_EPOCH};

// ─── Identifiers ───────────────────────────────────────────────────────────────

/// Unique identifier for a background worker instance.
/// Stable across restarts within a session. Used in logs and health reports.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkerId(pub String);

impl WorkerId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

/// Unique identifier for a per-display analysis worker.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisplayWorkerId(pub String);

impl DisplayWorkerId {
    pub fn new(display_id: &str) -> Self {
        Self(format!("display_worker_{}", display_id))
    }
}

/// Unique identifier for any registered Service.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceId(pub String);

impl ServiceId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

// ─── Worker State Machine ───────────────────────────────────────────────────────

/// State machine for the BackgroundWorker lifecycle.
///
/// ## Valid Transitions
/// ```text
/// Initializing   → Running         (startup complete)
/// Running        → Paused          (manual pause or OS lock screen)
/// Running        → Sleeping        (OS sleep / power event)
/// Running        → Recovering      (watchdog restart triggered)
/// Running        → Stopping        (ServiceManager::stop called)
/// Paused         → Running         (manual resume)
/// Sleeping       → Running         (OS wake + stabilization delay)
/// Recovering     → Running         (restart succeeded)
/// Recovering     → Stopped         (max_worker_restarts exceeded)
/// Stopping       → Stopped         (all threads joined)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum WorkerState {
    Initializing,
    Running,
    Paused,
    Sleeping,
    Recovering,
    Stopping,
    Stopped,
}

impl std::fmt::Display for WorkerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "Initializing"),
            Self::Running => write!(f, "Running"),
            Self::Paused => write!(f, "Paused"),
            Self::Sleeping => write!(f, "Sleeping"),
            Self::Recovering => write!(f, "Recovering"),
            Self::Stopping => write!(f, "Stopping"),
            Self::Stopped => write!(f, "Stopped"),
        }
    }
}

// ─── Worker Health ──────────────────────────────────────────────────────────────

/// Health snapshot read by the Dashboard and the Watchdog.
///
/// ## Heartbeat vs. Last Cycle
/// - `last_heartbeat_ms`: updated every loop iteration (even when no pipeline runs).
///   The Watchdog uses this to detect frozen workers.
/// - `last_cycle_ms`: updated only after a complete pipeline execution.
///   The Dashboard uses this to show "last updated" time.
#[derive(Debug, Clone)]
pub struct WorkerHealth {
    pub worker_id: WorkerId,
    pub running: bool,
    pub current_state: WorkerState,
    /// Updated every loop — used by Watchdog for frozen detection.
    pub last_heartbeat_ms: u64,
    /// Updated only after a successful pipeline cycle — used by Dashboard.
    pub last_cycle_ms: Option<u64>,
    /// Updated only after a cycle completes without subsystem errors.
    pub last_success_ms: Option<u64>,
    pub restart_count: u32,
    pub cpu_budget_ok: bool,
    pub memory_budget_ok: bool,
    pub current_poll_interval_ms: u64,
    /// Cumulative non-fatal subsystem errors (not crashes).
    pub error_count: u32,
}

impl WorkerHealth {
    pub fn initial(worker_id: WorkerId, poll_interval_ms: u64) -> Self {
        Self {
            worker_id,
            running: false,
            current_state: WorkerState::Initializing,
            last_heartbeat_ms: now_ms(),
            last_cycle_ms: None,
            last_success_ms: None,
            restart_count: 0,
            cpu_budget_ok: true,
            memory_budget_ok: true,
            current_poll_interval_ms: poll_interval_ms,
            error_count: 0,
        }
    }
}

// ─── Pipeline Profile & Result ──────────────────────────────────────────────────

/// Per-stage timing for the most recent pipeline cycle.
/// Stores only the latest values. No history. No memory growth.
#[derive(Debug, Clone, Default)]
pub struct PipelineProfile {
    pub ambient_ms: u64,
    pub screen_analysis_ms: u64,
    pub comfort_matching_ms: u64,
    pub visual_comfort_ms: u64,
    pub brightness_ms: u64,
    pub transition_ms: u64,
    pub total_ms: u64,
}

/// Result returned by every pipeline execution.
/// Used for diagnostics and debugging. Not persisted.
#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub success: bool,
    /// Human-readable outcome description.
    pub reason: String,
    pub duration_ms: u64,
    pub changed_brightness: bool,
    /// If the cycle was skipped or partially skipped, why.
    pub skipped_reason: Option<String>,
    /// Number of subsystem errors encountered during this cycle.
    pub error_count: u32,
}

// ─── Background Diagnostics ─────────────────────────────────────────────────────

/// Self-diagnostics snapshot for debugging and future Dashboard integration.
/// Readable at any time without blocking the worker loop.
#[derive(Debug, Clone)]
pub struct BackgroundDiagnostics {
    pub worker_alive: bool,
    pub queue_depth: usize,
    pub display_count: usize,
    pub scheduler_interval_ms: u64,
    pub watchdog_running: bool,
    /// Future: detect potential deadlock via missed heartbeat chain.
    pub future_deadlock_detected: bool,
}

// ─── Helpers ───────────────────────────────────────────────────────────────────

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
