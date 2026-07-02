use crate::visual_comfort::engine::VisualComfortEngine;
use crate::visual_comfort::filters::rate_limiter::DefaultRateLimiter;
use crate::visual_comfort::filters::stabilizer::DefaultComfortStabilizer;
use crate::visual_comfort::models::ComfortConfig;
use crate::visual_comfort::strategies::basic::BasicCompensationStrategy;

pub fn create_visual_comfort_engine(config: ComfortConfig) -> VisualComfortEngine {
    let stabilizer = Box::new(DefaultComfortStabilizer::new());
    let strategy = Box::new(BasicCompensationStrategy::new());
    let rate_limiter = Box::new(DefaultRateLimiter::new());

    VisualComfortEngine::new(config, stabilizer, strategy, rate_limiter)
}
