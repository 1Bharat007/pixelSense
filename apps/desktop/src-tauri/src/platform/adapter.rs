use crate::display::domain::{DisplayCapabilities, DisplayInfo};
use super::models::NativeDisplay;

/// Converts the internal NativeDisplay into the domain's DisplayInfo.
impl From<NativeDisplay> for DisplayInfo {
    fn from(native: NativeDisplay) -> Self {
        Self {
            id: native.id,
            name: native.name,
            manufacturer: None, // Not acquired in native discovery yet
            model: None, // Not acquired in native discovery yet
            width: native.width,
            height: native.height,
            refresh_rate: native.refresh_rate,
            is_primary: native.is_primary,
            capabilities: DisplayCapabilities {
                brightness: true,
                hdr: native.hdr_supported,
                ddc_ci: !native.is_internal,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::display::domain::DisplayInfo;
    use crate::platform::models::NativeDisplay;

    #[test]
    fn test_native_display_to_display_info_adapter() {
        let native = NativeDisplay {
            id: "display1".to_string(),
            name: "Generic PnP Monitor".to_string(),
            width: 1920,
            height: 1080,
            position_x: 0,
            position_y: 0,
            refresh_rate: Some(144.0),
            is_primary: true,
            hdr_supported: true,
            scaling_factor: 1.5,
            is_internal: false,
        };

        let info: DisplayInfo = native.into();

        assert_eq!(info.id, "display1");
        assert_eq!(info.name, "Generic PnP Monitor");
        assert_eq!(info.width, 1920);
        assert_eq!(info.height, 1080);
        assert_eq!(info.refresh_rate, Some(144.0));
        assert_eq!(info.is_primary, true);
        assert_eq!(info.manufacturer, None);
        assert_eq!(info.model, None);
        assert_eq!(info.capabilities.hdr, true);
        assert_eq!(info.capabilities.ddc_ci, true);
    }
}
