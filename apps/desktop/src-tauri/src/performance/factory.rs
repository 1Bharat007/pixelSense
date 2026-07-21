use crate::performance::config::PerformanceConfig;
use crate::performance::manager::PerformanceManager;
use crate::performance::power::WindowsPowerAnalyzer;
use crate::performance::window::WindowsWindowAnalyzer;

pub fn create_performance_manager(config: PerformanceConfig) -> PerformanceManager {
    PerformanceManager::new(
        config,
        Box::new(WindowsPowerAnalyzer),
        Box::new(WindowsWindowAnalyzer),
    )
}
