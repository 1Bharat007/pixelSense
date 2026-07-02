/// Background Adaptive Service configuration.
///
/// All tunable parameters are centralized here.
/// `config_version` ensures future migrations can detect schema changes.
#[derive(Debug, Clone)]
pub struct BackgroundConfig {
    /// Schema version. Increment when fields are added or renamed.
    pub config_version: u32,

    /// Master switch. If false, ServiceManager will not start any workers.
    pub enabled: bool,

    /// Base poll interval in milliseconds. Default: 500ms.
    pub base_poll_interval_ms: u64,

    /// Minimum poll interval enforced by PollingScheduler. Default: 100ms.
    pub minimum_poll_interval_ms: u64,

    /// Maximum poll interval enforced by PollingScheduler (backoff ceiling). Default: 5000ms.
    pub maximum_poll_interval_ms: u64,

    /// When true, PollingScheduler backs off when no changes are detected.
    pub adaptive_scheduling_enabled: bool,

    /// When true, the worker transitions to Sleeping on OS sleep events.
    pub pause_when_sleeping: bool,

    /// How long the Watchdog waits before declaring a worker frozen (milliseconds).
    pub watchdog_timeout_ms: u64,

    /// Maximum times the Watchdog will restart a worker before giving up.
    pub max_worker_restarts: u32,

    /// Stabilization delay after waking from OS sleep (milliseconds).
    pub wake_stabilization_delay_ms: u64,

    /// Current power mode. Only `Balanced` is active. Others are documented for future use.
    pub power_mode: PowerMode,
}

impl Default for BackgroundConfig {
    fn default() -> Self {
        Self {
            config_version: 1,
            enabled: true,
            base_poll_interval_ms: 500,
            minimum_poll_interval_ms: 100,
            maximum_poll_interval_ms: 5000,
            adaptive_scheduling_enabled: true,
            pause_when_sleeping: true,
            watchdog_timeout_ms: 10_000,
            max_worker_restarts: 5,
            wake_stabilization_delay_ms: 2000,
            power_mode: PowerMode::Balanced,
        }
    }
}

/// Power mode controls the aggressiveness of polling and CPU usage.
///
/// ## Future Implementation Note
/// Only `Balanced` is active in this sprint.
/// `Performance` and `BatterySaver` are stubs for future integration
/// with Windows power notifications (`GUID_POWERSCHEME_PERSONALITY`).
#[derive(Debug, Clone, PartialEq)]
pub enum PowerMode {
    /// Lower poll intervals, faster reaction. Higher CPU budget.
    /// Future: activate when plugged in and high-performance scheme detected.
    Performance,

    /// Default. 500ms base interval. Suitable for most desktop use.
    Balanced,

    /// Higher intervals (up to 2000ms base). Reduced analysis frequency.
    /// Future: activate on battery below threshold or OS battery saver mode.
    BatterySaver,
}
