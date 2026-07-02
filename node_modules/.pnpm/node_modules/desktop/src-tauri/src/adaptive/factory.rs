use std::sync::{Arc, Mutex};
use crate::adaptive::config::AdaptiveConfig;
use crate::adaptive::service::AdaptiveBrightnessService;
use crate::adaptive::state::BrightnessState;
use crate::decision::factory::create_decision_manager;
use crate::transition::factory::create_transition_manager;
use crate::brightness::manager::BrightnessManager;

pub fn create_adaptive_service(brightness_manager: Arc<BrightnessManager>) -> AdaptiveBrightnessService {
    let decision_manager = create_decision_manager();
    let transition_manager = create_transition_manager(brightness_manager);
    
    AdaptiveBrightnessService::new(
        decision_manager,
        transition_manager,
        AdaptiveConfig::default(),
        Arc::new(Mutex::new(BrightnessState::new())),
    )
}
