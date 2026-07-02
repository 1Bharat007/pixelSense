use crate::capabilities::error::CapabilityError;
use crate::capabilities::providers::CapabilityProvider;
use crate::display::domain::{DisplayCapabilities, DisplayInfo};

/// CapabilityManager orchestrates the evaluation of displays.
/// 
/// Future responsibilities (TODO):
/// - Capability caching
/// - Refreshing capabilities on hot-plug events
/// - Cache invalidation
/// - Emitting notifications on capability change
pub struct CapabilityManager {
    provider: Box<dyn CapabilityProvider>,
}

impl CapabilityManager {
    pub fn new(provider: Box<dyn CapabilityProvider>) -> Self {
        Self { provider }
    }

    pub fn evaluate(&self, display: &DisplayInfo) -> Result<DisplayCapabilities, CapabilityError> {
        self.provider.evaluate(display)
    }
}
