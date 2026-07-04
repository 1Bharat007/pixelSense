pub mod basic;

use crate::visual_comfort::models::{ComfortConfig, VisualComfortContext, VisualComfortResult};

pub trait CompensationStrategy: Send + Sync {
    fn calculate_compensation(
        &self,
        context: &VisualComfortContext,
        config: &ComfortConfig,
    ) -> VisualComfortResult;
}
