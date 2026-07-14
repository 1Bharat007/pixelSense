use crate::display::domain::{DisplayCapabilities, DisplayError, DisplayInfo};
use crate::display::providers::DisplayProvider;

pub enum MockScenario {
    LaptopOnly,
    LaptopAndExternal,
    TripleWorkstation,
    Empty,
}

pub struct MockProvider {
    scenario: MockScenario,
}

impl MockProvider {
    pub fn new(scenario: MockScenario) -> Self {
        Self { scenario }
    }
}

impl DisplayProvider for MockProvider {
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, DisplayError> {
        match self.scenario {
            MockScenario::LaptopOnly => Ok(vec![DisplayInfo {
                id: "mock_laptop_1".into(),
                name: "Internal Display".into(),
                manufacturer: Some("Mock".into()),
                model: None,
                width: 1920,
                height: 1080,
                refresh_rate: Some(60.0),
                is_primary: true,
                capabilities: DisplayCapabilities {
                    brightness: true,
                    hdr: false,
                    ddc_ci: false,
                },
            }]),
            MockScenario::Empty => Ok(vec![]),
            _ => Ok(vec![]), // simplify others for now
        }
    }
}
