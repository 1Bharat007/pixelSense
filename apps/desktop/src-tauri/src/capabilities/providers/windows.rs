use crate::capabilities::error::CapabilityError;
use crate::capabilities::providers::CapabilityProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

pub struct WindowsCapabilityProvider;

impl WindowsCapabilityProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowsCapabilityProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityProvider for WindowsCapabilityProvider {
    fn evaluate(&self, display: &DisplayInfo) -> Result<DisplayCapabilities, CapabilityError> {
        // In a future release, this will delegate to `crate::platform::factory::create_platform().discover_capabilities(display)`
        // For this sprint, we use deterministic placeholder capability profiles.
        let name = display.name.to_lowercase();
        
        if name.contains("internal") || name.contains("laptop") {
            Ok(DisplayCapabilities { brightness: true, hdr: false, ddc_ci: false })
        } else if name.contains("office") {
            Ok(DisplayCapabilities { brightness: true, hdr: false, ddc_ci: true })
        } else if name.contains("gaming") {
            Ok(DisplayCapabilities { brightness: true, hdr: true, ddc_ci: true })
        } else if name.contains("projector") {
            Ok(DisplayCapabilities { brightness: false, hdr: false, ddc_ci: false })
        } else {
            Ok(DisplayCapabilities { brightness: false, hdr: false, ddc_ci: false }) // Unknown display
        }
    }
}
