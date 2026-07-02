pub mod error;
pub mod factory;
pub mod manager;
pub mod providers;

#[cfg(test)]
mod tests {
    use super::factory::create_capability_manager;
    use crate::display::domain::{DisplayCapabilities, DisplayInfo};

    fn create_dummy_display(name: &str) -> DisplayInfo {
        DisplayInfo {
            id: format!("{}_id", name),
            name: name.to_string(),
            manufacturer: None,
            model: None,
            width: 1920,
            height: 1080,
            refresh_rate: None,
            is_primary: false,
            capabilities: DisplayCapabilities::default(),
        }
    }

    #[test]
    fn test_evaluate_internal_laptop() {
        let manager = create_capability_manager();
        let display = create_dummy_display("Internal Laptop Screen");
        let caps = manager.evaluate(&display).unwrap();
        assert_eq!(caps.brightness, true);
        assert_eq!(caps.hdr, false);
        assert_eq!(caps.ddc_ci, false);
    }

    #[test]
    fn test_evaluate_office_monitor() {
        let manager = create_capability_manager();
        let display = create_dummy_display("Dell Office Monitor");
        let caps = manager.evaluate(&display).unwrap();
        assert_eq!(caps.brightness, true);
        assert_eq!(caps.hdr, false);
        assert_eq!(caps.ddc_ci, true);
    }

    #[test]
    fn test_evaluate_gaming_monitor() {
        let manager = create_capability_manager();
        let display = create_dummy_display("ASUS Gaming Display");
        let caps = manager.evaluate(&display).unwrap();
        assert_eq!(caps.brightness, true);
        assert_eq!(caps.hdr, true);
        assert_eq!(caps.ddc_ci, true);
    }

    #[test]
    fn test_evaluate_projector() {
        let manager = create_capability_manager();
        let display = create_dummy_display("Epson Projector");
        let caps = manager.evaluate(&display).unwrap();
        assert_eq!(caps.brightness, false);
        assert_eq!(caps.hdr, false);
        assert_eq!(caps.ddc_ci, false);
    }

    #[test]
    fn test_evaluate_unknown_display() {
        let manager = create_capability_manager();
        let display = create_dummy_display("Unknown Generic Device");
        let caps = manager.evaluate(&display).unwrap();
        assert_eq!(caps.brightness, false);
        assert_eq!(caps.hdr, false);
        assert_eq!(caps.ddc_ci, false);
    }

    #[test]
    fn test_multiple_displays() {
        let manager = create_capability_manager();
        let displays = vec![
            create_dummy_display("Laptop"),
            create_dummy_display("Office"),
        ];

        let results: Vec<_> = displays.iter().map(|d| manager.evaluate(d).unwrap()).collect();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].ddc_ci, false); // Laptop
        assert_eq!(results[1].ddc_ci, true);  // Office
    }

    #[test]
    fn test_empty_display_list() {
        let manager = create_capability_manager();
        let displays: Vec<DisplayInfo> = vec![];
        let results: Vec<_> = displays.iter().map(|d| manager.evaluate(d).unwrap()).collect();
        assert_eq!(results.len(), 0);
    }
}
