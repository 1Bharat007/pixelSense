use crate::brightness::manager::BrightnessManager;
use crate::brightness::providers::native::NativeBrightnessProvider;
use crate::brightness::providers::BrightnessProvider;

#[cfg(test)]
use crate::brightness::providers::mock::MockBrightnessProvider;

pub struct BrightnessFactory;

impl BrightnessFactory {
    pub fn create_default() -> BrightnessManager {
        let provider = Box::new(NativeBrightnessProvider::new());
        BrightnessManager::new(provider)
    }
}

pub fn create_brightness_manager() -> BrightnessManager {
    BrightnessManager::new(create_provider())
}

fn create_provider() -> Box<dyn BrightnessProvider> {
    #[cfg(test)]
    {
        Box::new(MockBrightnessProvider::new())
    }
    #[cfg(not(test))]
    {
        Box::new(NativeBrightnessProvider::new())
    }
}
