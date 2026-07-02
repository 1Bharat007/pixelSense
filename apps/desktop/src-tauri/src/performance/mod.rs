pub mod config;
pub mod diagnostics;
pub mod factory;
pub mod manager;
pub mod models;
pub mod power;
pub mod window;

#[cfg(test)]
mod tests {
    use crate::performance::config::PerformanceConfig;
    use crate::performance::manager::PerformanceManager;
    use crate::performance::models::PowerState;
    use crate::performance::power::MockPowerAnalyzer;
    use crate::performance::window::MockWindowAnalyzer;

    fn create_mock_manager() -> (PerformanceManager, Box<MockPowerAnalyzer>, Box<MockWindowAnalyzer>) {
        let power_analyzer = Box::new(MockPowerAnalyzer::new(PowerState::AC));
        let window_analyzer = Box::new(MockWindowAnalyzer::new(false));
        
        let power_ref = Box::new(MockPowerAnalyzer::new(PowerState::AC));
        let window_ref = Box::new(MockWindowAnalyzer::new(false));
        
        let manager = PerformanceManager::new(
            PerformanceConfig::default(),
            power_analyzer,
            window_analyzer,
        );
        (manager, power_ref, window_ref) // Returning refs for ease of mocking if needed
    }

    #[test]
    fn test_power_state_intervals() {
        // Test AC
        let power_analyzer = Box::new(MockPowerAnalyzer::new(PowerState::AC));
        let window_analyzer = Box::new(MockWindowAnalyzer::new(false));
        let manager = PerformanceManager::new(PerformanceConfig::default(), power_analyzer, window_analyzer);
        let state = manager.evaluate_performance_state();
        assert_eq!(state.active_policy.screen_analysis_interval_ms, 500);

        // Test BatteryLow
        let power_analyzer_low = Box::new(MockPowerAnalyzer::new(PowerState::BatteryLow));
        let window_analyzer_low = Box::new(MockWindowAnalyzer::new(false));
        let manager_low = PerformanceManager::new(PerformanceConfig::default(), power_analyzer_low, window_analyzer_low);
        let state_low = manager_low.evaluate_performance_state();
        assert_eq!(state_low.active_policy.screen_analysis_interval_ms, 2500);
        assert_eq!(state_low.active_policy.ambient_interval_ms, 1500);
    }

    #[test]
    fn test_fullscreen_option_b_policy() {
        let power_analyzer = Box::new(MockPowerAnalyzer::new(PowerState::AC));
        let window_analyzer = Box::new(MockWindowAnalyzer::new(true)); // Fullscreen active
        let manager = PerformanceManager::new(PerformanceConfig::default(), power_analyzer, window_analyzer);
        
        let state = manager.evaluate_performance_state();
        assert!(state.active_policy.pause_screen_analysis);
        assert!(!state.active_policy.pause_ambient); // Ambient should continue
    }

    #[test]
    fn test_static_screen_backoff() {
        let power_analyzer = Box::new(MockPowerAnalyzer::new(PowerState::AC));
        let window_analyzer = Box::new(MockWindowAnalyzer::new(false));
        let manager = PerformanceManager::new(PerformanceConfig::default(), power_analyzer, window_analyzer);
        
        manager.report_screen_changed(false);
        manager.report_screen_changed(false);
        manager.report_screen_changed(false);
        
        let state = manager.evaluate_performance_state();
        assert!(state.active_policy.screen_analysis_interval_ms > 500); // Should be backed off
        
        // Reset
        manager.report_screen_changed(true);
        let reset_state = manager.evaluate_performance_state();
        assert_eq!(reset_state.active_policy.screen_analysis_interval_ms, 500);
    }
}
