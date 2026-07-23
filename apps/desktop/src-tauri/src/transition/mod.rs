pub mod config;
pub mod error;
pub mod factory;
pub mod interpolator;
pub mod manager;
pub mod providers;
pub mod worker;

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::brightness::manager::BrightnessManager;
    use crate::brightness::providers::mock::MockBrightnessProvider;
    use crate::display::domain::{DisplayCapabilities, DisplayInfo};
    use crate::transition::config::TransitionConfig;
    use crate::transition::manager::TransitionManager;
    use crate::transition::providers::mock::MockTransitionProvider;

    fn create_dummy_display() -> (DisplayInfo, DisplayCapabilities) {
        let caps = DisplayCapabilities { brightness: true, hdr: false, ddc_ci: false };
        let display = DisplayInfo {
            id: "laptop_id".into(),
            name: "Laptop".into(),
            manufacturer: None,
            model: None,
            width: 1920,
            height: 1080,
            refresh_rate: None,
            is_primary: true,
            capabilities: caps.clone(),
        };
        (display, caps)
    }

    fn setup() -> (Arc<BrightnessManager>, MockTransitionProvider, TransitionManager) {
        let brightness_provider = Box::new(MockBrightnessProvider::new());
        let brightness_manager = Arc::new(BrightnessManager::new(brightness_provider));
        
        let mock_provider = MockTransitionProvider::new();
        let manager = TransitionManager::new(
            Box::new(mock_provider.clone()),
            Arc::clone(&brightness_manager),
            TransitionConfig::default(),
        );

        (brightness_manager, mock_provider, manager)
    }

    #[test]
    fn test_increasing_brightness() {
        let (_bm, provider, manager) = setup();
        let (display, caps) = create_dummy_display();

        manager.transition_brightness(&display, &caps, 10, 50, 100, crate::transition::manager::ExecutionMode::Transition).unwrap();

        let records = provider.records.lock().unwrap();
        assert!(records.len() > 1);
        assert_eq!(records.first().unwrap().brightness, 17); // round(10 + 40/6) = 17
        assert_eq!(records.last().unwrap().brightness, 50); // Target
    }

    #[test]
    fn test_decreasing_brightness() {
        let (_bm, provider, manager) = setup();
        let (display, caps) = create_dummy_display();

        manager.transition_brightness(&display, &caps, 100, 0, 50, crate::transition::manager::ExecutionMode::Transition).unwrap();

        let records = provider.records.lock().unwrap();
        assert!(records.len() > 1);
        assert_eq!(records.last().unwrap().brightness, 0); // Target
    }

    #[test]
    fn test_same_brightness() {
        let (_bm, provider, manager) = setup();
        let (display, caps) = create_dummy_display();

        manager.transition_brightness(&display, &caps, 50, 50, 100, crate::transition::manager::ExecutionMode::Transition).unwrap();

        let records = provider.records.lock().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].brightness, 50);
        assert_eq!(records[0].simulated_timestamp_ms, 0);
    }

    #[test]
    fn test_invalid_duration_zero() {
        let (_bm, provider, manager) = setup();
        let (display, caps) = create_dummy_display();

        manager.transition_brightness(&display, &caps, 10, 80, 0, crate::transition::manager::ExecutionMode::Transition).unwrap();

        let records = provider.records.lock().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].brightness, 80); // Immediate
    }

    #[test]
    fn test_long_duration() {
        let (_bm, provider, manager) = setup();
        let (display, caps) = create_dummy_display();

        // 10 seconds duration
        manager.transition_brightness(&display, &caps, 0, 100, 10000, crate::transition::manager::ExecutionMode::Transition).unwrap();

        let records = provider.records.lock().unwrap();
        assert_eq!(records.len(), 625); // 10000 / 16
        assert_eq!(records.last().unwrap().brightness, 100);
    }

    #[test]
    fn test_very_short_duration() {
        let (_bm, provider, manager) = setup();
        let (display, caps) = create_dummy_display();

        // Duration shorter than tick interval
        manager.transition_brightness(&display, &caps, 10, 20, 5, crate::transition::manager::ExecutionMode::Transition).unwrap();

        let records = provider.records.lock().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records.last().unwrap().brightness, 20);
    }

    #[test]
    fn test_simulated_timestamps() {
        let (_bm, provider, manager) = setup();
        let (display, caps) = create_dummy_display();

        manager.transition_brightness(&display, &caps, 0, 10, 32, crate::transition::manager::ExecutionMode::Transition).unwrap();

        let records = provider.records.lock().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].simulated_timestamp_ms, 16);
        assert_eq!(records[1].simulated_timestamp_ms, 32);
    }

    #[test]
    fn test_interrupted_transition() {
        let (_bm, provider, manager) = setup();
        let (display, caps) = create_dummy_display();

        *provider.interrupted.lock().unwrap() = true;

        manager.transition_brightness(&display, &caps, 0, 100, 100, crate::transition::manager::ExecutionMode::Transition).unwrap();

        let records = provider.records.lock().unwrap();
        // Since interrupted is true, the loop breaks immediately.
        assert_eq!(records.len(), 0);
    }
}

