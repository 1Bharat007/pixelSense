use crate::intelligence::models::IntelligenceContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComfortScoreResult {
    pub total_score: u8,
    pub environment_component: u8,
    pub screen_component: u8,
    pub behavior_component: u8,
    pub transition_component: u8,
    pub confidence_component: u8,
}

pub struct ComfortScoreEngine;

impl ComfortScoreEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate(&self, context: &IntelligenceContext) -> ComfortScoreResult {
        // 40% Environment (Based on ambient lux stability and appropriate brightness)
        // Placeholder logic: assume 90% stability
        let env_score = 36; // out of 40

        // 25% Screen Stability (Based on visual complexity and content changes)
        let screen_score = 22; // out of 25

        // 15% User Behaviour (Based on manual overrides in history)
        // More overrides = lower score
        let overrides = context.history_summary.manual_overrides_today;
        let behavior_score = if overrides > 10 { 5 } else { 15 - overrides as u8 }; // out of 15

        // 10% Transitions (Smoothness, frequency)
        let transition_score = 9; // out of 10

        // 10% Confidence
        let conf_score = (context.confidence_score * 10.0) as u8; // out of 10

        let total = env_score + screen_score + behavior_score + transition_score + conf_score;

        ComfortScoreResult {
            total_score: total.min(100),
            environment_component: env_score,
            screen_component: screen_score,
            behavior_component: behavior_score,
            transition_component: transition_score,
            confidence_component: conf_score,
        }
    }
}
