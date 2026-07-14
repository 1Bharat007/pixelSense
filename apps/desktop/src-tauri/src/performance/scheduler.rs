use std::time::Instant;
use crate::performance::budget::PerformanceBudgetManager;
use std::sync::Arc;

/// CentralScheduler coordinates all periodic tasks (Ambient, Screen Analysis, Dashboard)
/// to ensure there is only one polling engine driving the background worker.
pub struct CentralScheduler {
    budget_manager: Arc<PerformanceBudgetManager>,
    last_ambient_ms: u64,
    last_screen_ms: u64,
    #[allow(dead_code)] // Reserved for future dashboard coordination
    last_dashboard_ms: u64,
    start_time: Instant,
}

impl CentralScheduler {
    pub fn new(budget_manager: Arc<PerformanceBudgetManager>) -> Self {
        Self {
            budget_manager,
            last_ambient_ms: 0,
            last_screen_ms: 0,
            last_dashboard_ms: 0,
            start_time: Instant::now(),
        }
    }

    fn now_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }

    pub fn should_poll_ambient(&mut self) -> bool {
        let now = self.now_ms();
        let interval = if self.budget_manager.is_throttled() { 1000 } else { 200 };
        if now - self.last_ambient_ms >= interval {
            self.last_ambient_ms = now;
            true
        } else {
            false
        }
    }

    pub fn should_poll_screen(&mut self) -> bool {
        let now = self.now_ms();
        let interval = if self.budget_manager.is_throttled() { 2000 } else { 500 };
        if now - self.last_screen_ms >= interval {
            self.last_screen_ms = now;
            true
        } else {
            false
        }
    }
}
