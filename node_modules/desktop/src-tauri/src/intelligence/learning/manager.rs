use crate::intelligence::models::IntelligenceContext;
use crate::intelligence::behavior::models::UserBehaviorSnapshot;
use crate::intelligence::learning::models::LearningObservation;

pub struct LearningEngine;

impl LearningEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_observations(
        &self,
        context: &IntelligenceContext,
        behavior: &UserBehaviorSnapshot,
    ) -> Vec<LearningObservation> {
        let mut observations = Vec::new();

        // Detect basic patterns
        if behavior.session_duration_minutes > 120 {
            observations.push(LearningObservation::Pattern {
                context: "Long Session".into(),
                behavior: "Deep Focus".into(),
            });
        }

        if behavior.manual_overrides_today > 5 {
            observations.push(LearningObservation::Anomaly {
                description: "Unusually high manual brightness overrides".into(),
                severity: "Medium".into(),
            });
        }

        // Trends based on context
        if context.history_summary.brightness_changes_today > 20 {
            observations.push(LearningObservation::Trend {
                metric: "Brightness Adjustments".into(),
                direction: "Increasing".into(),
            });
        }

        observations
    }
}
