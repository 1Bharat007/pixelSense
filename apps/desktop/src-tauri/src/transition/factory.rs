use std::sync::Arc;
use crate::brightness::manager::BrightnessManager;
use crate::transition::config::TransitionConfig;
use crate::transition::manager::TransitionManager;
use crate::transition::providers::TransitionProvider;
#[cfg(not(test))]
use crate::transition::providers::default::DefaultTransitionProvider;

#[cfg(test)]
use crate::transition::providers::mock::MockTransitionProvider;

pub fn create_transition_manager(brightness_manager: Arc<BrightnessManager>) -> TransitionManager {
    TransitionManager::new(create_provider(), brightness_manager, TransitionConfig::default())
}

fn create_provider() -> Box<dyn TransitionProvider> {
    #[cfg(test)]
    {
        Box::new(MockTransitionProvider::new())
    }
    #[cfg(not(test))]
    {
        Box::new(DefaultTransitionProvider::new())
    }
}
