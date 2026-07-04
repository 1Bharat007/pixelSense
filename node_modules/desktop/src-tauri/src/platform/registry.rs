use crate::platform::capabilities::PlatformCapabilities;
use std::sync::RwLock;

/// Centralized registry for all hardware and platform capabilities.
/// Replaces independent hardware checks by each subsystem.
pub struct CapabilityRegistry {
    capabilities: RwLock<PlatformCapabilities>,
}

impl CapabilityRegistry {
    pub fn new(initial: PlatformCapabilities) -> Self {
        Self {
            capabilities: RwLock::new(initial),
        }
    }

    pub fn get(&self) -> PlatformCapabilities {
        self.capabilities.read().unwrap().clone()
    }

    pub fn update(&self, new_caps: PlatformCapabilities) {
        let mut caps = self.capabilities.write().unwrap();
        *caps = new_caps;
    }
}
