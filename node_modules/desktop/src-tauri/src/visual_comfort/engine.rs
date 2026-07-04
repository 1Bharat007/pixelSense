use crate::visual_comfort::filters::rate_limiter::RateLimiter;
use crate::visual_comfort::filters::stabilizer::ComfortStabilizer;
use crate::visual_comfort::models::{ComfortConfig, VisualComfortContext, VisualComfortResult};
use crate::visual_comfort::strategies::CompensationStrategy;
use std::sync::Mutex;

pub struct VisualComfortEngine {
    pub config: ComfortConfig,
    stabilizer: Box<dyn ComfortStabilizer>,
    strategy: Box<dyn CompensationStrategy>,
    rate_limiter: Mutex<Box<dyn RateLimiter>>,
}

impl VisualComfortEngine {
    pub fn new(
        config: ComfortConfig,
        stabilizer: Box<dyn ComfortStabilizer>,
        strategy: Box<dyn CompensationStrategy>,
        rate_limiter: Box<dyn RateLimiter>,
    ) -> Self {
        Self {
            config,
            stabilizer,
            strategy,
            rate_limiter: Mutex::new(rate_limiter),
        }
    }

    pub fn calculate_comfort(&self, mut context: VisualComfortContext) -> VisualComfortResult {
        // 1. Stabilize input luminance (prevent rapid oscillation)
        if let Some(luminance) = context.screen_luminance {
            context.screen_luminance = Some(self.stabilizer.stabilize(luminance, &self.config));
        }

        // 2. Calculate ideal target using strategy
        let mut result = self.strategy.calculate_compensation(&context, &self.config);

        // 3. Rate limiting and threshold filtering
        let mut limiter = self.rate_limiter.lock().unwrap();
        let target = result.recommendation.recommended_brightness;
        
        if result.recommendation.action != crate::visual_comfort::models::RecommendationAction::NoChange {
            if !limiter.should_update(&context.display_id, context.current_monitor_brightness, target, &self.config) {
                result.recommendation.action = crate::visual_comfort::models::RecommendationAction::Ignore;
                result.recommendation.reason = "Rate limited or below threshold".into();
            } else {
                limiter.record_update(&context.display_id);
            }
        }

        result
    }
}
