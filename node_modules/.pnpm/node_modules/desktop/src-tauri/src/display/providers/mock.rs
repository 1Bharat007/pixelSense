use crate::display::domain::{DisplayCapabilities, DisplayError, DisplayInfo};
use super::DisplayProvider;

pub struct MockProvider {
    pub scenario: MockScenario,
}

#[derive(Clone, Copy)]
pub enum MockScenario {
    LaptopOnly,
    LaptopAndExternal,
    TripleWorkstation,
    Empty,
}

impl MockProvider {
    pub fn new(scenario: MockScenario) -> Self {
        Self { scenario }
    }
}

impl DisplayProvider for MockProvider {
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, DisplayError> {
        match self.scenario {
            MockScenario::Empty => Ok(vec![]),
            MockScenario::LaptopOnly => Ok(vec![DisplayInfo {
                id: "mock_laptop_1".to_string(),
                name: "Internal Display".to_string(),
                manufacturer: Some("Generic".to_string()),
                model: Some("LCD1".to_string()),
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
            MockScenario::LaptopAndExternal => Ok(vec![
                DisplayInfo {
                    id: "mock_laptop_1".to_string(),
                    name: "Internal Display".to_string(),
                    manufacturer: Some("Generic".to_string()),
                    model: Some("LCD1".to_string()),
                    width: 1920,
                    height: 1080,
                    refresh_rate: Some(60.0),
                    is_primary: true,
                    capabilities: DisplayCapabilities {
                        brightness: true,
                        hdr: false,
                        ddc_ci: false,
                    },
                },
                DisplayInfo {
                    id: "mock_ext_1".to_string(),
                    name: "External Monitor".to_string(),
                    manufacturer: Some("Dell".to_string()),
                    model: Some("U2720Q".to_string()),
                    width: 3840,
                    height: 2160,
                    refresh_rate: Some(60.0),
                    is_primary: false,
                    capabilities: DisplayCapabilities {
                        brightness: true,
                        hdr: true,
                        ddc_ci: true,
                    },
                },
            ]),
            MockScenario::TripleWorkstation => Ok(vec![
                DisplayInfo {
                    id: "mock_ext_1".to_string(),
                    name: "External Left".to_string(),
                    manufacturer: Some("LG".to_string()),
                    model: Some("27GL850".to_string()),
                    width: 2560,
                    height: 1440,
                    refresh_rate: Some(144.0),
                    is_primary: false,
                    capabilities: DisplayCapabilities {
                        brightness: true,
                        hdr: true,
                        ddc_ci: true,
                    },
                },
                DisplayInfo {
                    id: "mock_ext_2".to_string(),
                    name: "External Center".to_string(),
                    manufacturer: Some("Dell".to_string()),
                    model: Some("U2720Q".to_string()),
                    width: 3840,
                    height: 2160,
                    refresh_rate: Some(60.0),
                    is_primary: true,
                    capabilities: DisplayCapabilities {
                        brightness: true,
                        hdr: true,
                        ddc_ci: true,
                    },
                },
                DisplayInfo {
                    id: "mock_ext_3".to_string(),
                    name: "External Right".to_string(),
                    manufacturer: Some("LG".to_string()),
                    model: Some("27GL850".to_string()),
                    width: 2560,
                    height: 1440,
                    refresh_rate: Some(144.0),
                    is_primary: false,
                    capabilities: DisplayCapabilities {
                        brightness: true,
                        hdr: true,
                        ddc_ci: true,
                    },
                },
            ]),
        }
    }
}

