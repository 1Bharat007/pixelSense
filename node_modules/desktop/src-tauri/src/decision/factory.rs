use crate::decision::config::DecisionConfig;
use crate::decision::manager::DecisionManager;
use crate::decision::strategies::default::DefaultDecisionStrategy;
use crate::decision::strategies::DecisionStrategy;

pub fn create_decision_manager() -> DecisionManager {
    DecisionManager::new(create_strategy(), DecisionConfig::default())
}

fn create_strategy() -> Box<dyn DecisionStrategy> {
    Box::new(DefaultDecisionStrategy::new())
}
