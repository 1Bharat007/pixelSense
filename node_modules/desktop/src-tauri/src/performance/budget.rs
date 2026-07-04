use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

/// PerformanceBudgetManager monitors the background worker's CPU, Memory, and Latency.
/// It dynamically throttles operations if limits are exceeded.
pub struct PerformanceBudgetManager {
    cpu_usage_pct: AtomicU32,
    memory_usage_mb: AtomicU32,
    is_throttled: AtomicBool,
}

impl PerformanceBudgetManager {
    pub fn new() -> Self {
        Self {
            cpu_usage_pct: AtomicU32::new(0),
            memory_usage_mb: AtomicU32::new(0),
            is_throttled: AtomicBool::new(false),
        }
    }

    /// Report current metrics to the budget manager
    pub fn report_metrics(&self, cpu: f32, mem_mb: u32) {
        self.cpu_usage_pct.store((cpu * 100.0) as u32, Ordering::Release);
        self.memory_usage_mb.store(mem_mb, Ordering::Release);
        
        // Target: <1% CPU and <50MB Memory
        let throttle = cpu > 1.0 || mem_mb > 50;
        self.is_throttled.store(throttle, Ordering::Release);
    }

    /// Check if the system is currently throttled due to exceeding performance budgets
    pub fn is_throttled(&self) -> bool {
        self.is_throttled.load(Ordering::Acquire)
    }
}
