use crate::intelligence::models::IntelligenceContext;
use crate::intelligence::insights::models::Insight;

pub struct InsightsEngine;

impl InsightsEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, context: &IntelligenceContext) -> Vec<Insight> {
        let mut insights = Vec::new();

        if context.history_summary.manual_overrides_today == 0 {
            insights.push(Insight {
                title: "Stable Comfort".into(),
                description: "No manual overrides required today. Adaptive engine is perfectly tuned.".into(),
                severity: "Low".into(),
                category: "Comfort".into(),
                confidence: 0.9,
                icon: "Eye".into(),
                timestamp: context.current_time_ms,
                related_events: vec![],
            });
        } else if context.history_summary.manual_overrides_today > 5 {
            insights.push(Insight {
                title: "Frequent Overrides".into(),
                description: format!("You changed brightness manually {} times today.", context.history_summary.manual_overrides_today),
                severity: "Medium".into(),
                category: "Comfort".into(),
                confidence: 0.85,
                icon: "Activity".into(),
                timestamp: context.current_time_ms,
                related_events: vec![],
            });
        }

        if context.active_application == "VSCode" {
            insights.push(Insight {
                title: "Coding Mode Active".into(),
                description: "Deep focus detected. Transition speeds have been slowed down.".into(),
                severity: "Low".into(),
                category: "Performance".into(),
                confidence: 0.95,
                icon: "Code".into(),
                timestamp: context.current_time_ms,
                related_events: vec![],
            });
        }

        if insights.is_empty() {
            insights.push(Insight {
                title: "Environment Stable".into(),
                description: "Lighting and screen conditions are optimal for your eyes.".into(),
                severity: "Low".into(),
                category: "Environment".into(),
                confidence: 0.9,
                icon: "Sun".into(),
                timestamp: context.current_time_ms,
                related_events: vec![],
            });
        }

        insights
    }
}
