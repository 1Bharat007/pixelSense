use crate::intelligence::models::IntelligenceContext;
use crate::intelligence::recommendations::models::Recommendation;
use crate::intelligence::behavior::models::UserBehaviorSnapshot;

pub struct RecommendationEngine;

impl RecommendationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(
        &self,
        context: &IntelligenceContext,
        behavior: &UserBehaviorSnapshot,
    ) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        if behavior.manual_overrides_today > 10 {
            recommendations.push(Recommendation {
                title: "Adjust Base Curve".into(),
                reason: "You are fighting the auto-brightness frequently today.".into(),
                priority: "High".into(),
                estimated_benefit: "Reduces need for manual overrides by 80%".into(),
                dismissable: true,
                action: "TUNE_BASE_CURVE".into(),
            });
        }

        if behavior.active_application == "Photoshop" && context.comfort_profile != "Color Critical" {
            recommendations.push(Recommendation {
                title: "Enable Color Accurate Mode".into(),
                reason: "Photoshop is active. Adaptive brightness may distort perceived colors.".into(),
                priority: "Medium".into(),
                estimated_benefit: "Ensures perfect color grading accuracy".into(),
                dismissable: true,
                action: "ENABLE_PROFILE_COLOR_CRITICAL".into(),
            });
        }

        if recommendations.is_empty() && context.current_ambient_lux < 20.0 {
            // Default placeholder recommendation for dark environments
            recommendations.push(Recommendation {
                title: "Try Night Owl Profile".into(),
                reason: "Your environment is very dark. Night Owl reduces blue light.".into(),
                priority: "Low".into(),
                estimated_benefit: "Improves sleep quality".into(),
                dismissable: true,
                action: "ENABLE_PROFILE_NIGHT_OWL".into(),
            });
        }

        recommendations
    }
}
