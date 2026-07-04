use crate::decision::config::DecisionConfig;
use crate::decision::error::DecisionError;
use crate::decision::models::{DecisionContext, DecisionResult};
use crate::decision::strategies::DecisionStrategy;

pub struct DecisionManager {
    strategy: Box<dyn DecisionStrategy>,
    config: DecisionConfig,
}

impl DecisionManager {
    pub fn new(strategy: Box<dyn DecisionStrategy>, config: DecisionConfig) -> Self {
        Self { strategy, config }
    }

    pub fn decide_brightness(&self, context: &DecisionContext) -> Result<DecisionResult, DecisionError> {
        // Delegate to strategy
        let mut result = self.strategy.calculate_brightness(context)?;

        // Apply global configuration limits
        result.recommended_brightness = result.recommended_brightness
            .clamp(self.config.minimum_brightness, self.config.maximum_brightness);

        Ok(result)
    }
}
