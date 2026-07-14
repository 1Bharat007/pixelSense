use crate::performance::config::PerformanceConfig;
use crate::performance::manager::PerformanceManager;
use crate::performance::power::WindowsPowerAnalyzer;
use crate::performance::window::WindowsWindowAnalyzer;
#[cfg(not(target_os = "windows"))]
use crate::performance::power::MockPowerAnalyzer;
#[cfg(not(target_os = "windows"))]
use crate::performance::window::MockWindowAnalyzer;

#[cfg(target_os = "windows")]
pub fn create_performance_manager(config: PerformanceConfig) -> PerformanceManager {
    PerformanceManager::new(
        config,
        Box::new(WindowsPowerAnalyzer),
        Box::new(WindowsWindowAnalyzer),
    )
}

#[cfg(not(target_os = "windows"))]
pub fn create_performance_manager(config: PerformanceConfig) -> PerformanceManager {
    PerformanceManager::new(
        config,
        Box::new(MockPowerAnalyzer::new(crate::performance::models::PowerState::AC)),
        Box::new(MockWindowAnalyzer::new(false)),
    )
}
