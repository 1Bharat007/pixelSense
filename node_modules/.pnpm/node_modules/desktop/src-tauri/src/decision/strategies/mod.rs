pub mod default;

use crate::decision::error::DecisionError;
use crate::decision::models::{DecisionContext, DecisionResult};

pub trait DecisionStrategy: Send + Sync {
    fn calculate_brightness(&self, context: &DecisionContext) -> Result<DecisionResult, DecisionError>;
}
