pub mod domain;
pub mod factory;
pub mod manager;
pub mod providers;
#[cfg(test)]
mod tests {
    use super::domain::{DisplayCapabilities, DisplayInfo};
    use super::manager::DisplayManager;
    use super::providers::mock::{MockProvider, MockScenario};
    use super::providers::DisplayProvider;
    use serde_json;

    #[test]
    fn test_mock_provider_laptop_only() {
        let provider = MockProvider::new(MockScenario::LaptopOnly);
        let displays = provider.get_displays().unwrap();
        assert_eq!(displays.len(), 1);
        assert_eq!(displays[0].id, "mock_laptop_1");
        assert!(displays[0].is_primary);
    }

    #[test]
    fn test_mock_provider_laptop_and_external() {
        let provider = MockProvider::new(MockScenario::LaptopAndExternal);
        let displays = provider.get_displays().unwrap();
        assert_eq!(displays.len(), 2);
    }

    #[test]
    fn test_mock_provider_triple_workstation() {
        let provider = MockProvider::new(MockScenario::TripleWorkstation);
        let displays = provider.get_displays().unwrap();
        assert_eq!(displays.len(), 3);
    }

    #[test]
    fn test_mock_provider_empty() {
        let provider = MockProvider::new(MockScenario::Empty);
        let displays = provider.get_displays().unwrap();
        assert_eq!(displays.len(), 0);
    }

    #[test]
    fn test_display_manager_delegation() {
        let provider = Box::new(MockProvider::new(MockScenario::LaptopOnly));
        let manager = DisplayManager::new(provider);
        let displays = manager.get_displays().unwrap();
        assert_eq!(displays.len(), 1);
    }

    #[test]
    fn test_serialization() {
        let display = DisplayInfo {
            id: "test_id".to_string(),
            name: "Test Display".to_string(),
            manufacturer: Some("Test Maker".to_string()),
            model: None,
            width: 1920,
            height: 1080,
            refresh_rate: Some(60.0),
            is_primary: true,
            capabilities: DisplayCapabilities {
                brightness: true,
                hdr: false,
                ddc_ci: true,
            },
        };

        let serialized = serde_json::to_string(&display).unwrap();
        let deserialized: DisplayInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(display, deserialized);
    }
}

