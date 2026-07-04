use crate::intelligence::models::IntelligenceContext;
use crate::intelligence::behavior::manager::BehaviorEngine;
use crate::intelligence::learning::manager::LearningEngine;
use crate::intelligence::analytics::manager::AnalyticsEngine;
use crate::intelligence::insights::manager::InsightsEngine;
use crate::intelligence::recommendations::manager::RecommendationEngine;
use crate::intelligence::comfort_score::manager::ComfortScoreEngine;
use crate::intelligence::application_rules::manager::ApplicationRuleEngine;
use crate::intelligence::comfort_score::manager::ComfortScoreResult;
use crate::intelligence::analytics::models::AnalyticsSnapshot;
use crate::intelligence::insights::models::Insight;
use crate::intelligence::recommendations::models::Recommendation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligencePayload {
    pub comfort_score: ComfortScoreResult,
    pub insights: Vec<Insight>,
    pub recommendations: Vec<Recommendation>,
    pub analytics: AnalyticsSnapshot,
}

pub struct IntelligenceManager {
    behavior_engine: BehaviorEngine,
    learning_engine: LearningEngine,
    analytics_engine: AnalyticsEngine,
    insights_engine: InsightsEngine,
    recommendation_engine: RecommendationEngine,
    comfort_score_engine: ComfortScoreEngine,
    application_rule_engine: ApplicationRuleEngine,
}

impl IntelligenceManager {
    pub fn new() -> Self {
        Self {
            behavior_engine: BehaviorEngine::new(),
            learning_engine: LearningEngine::new(),
            analytics_engine: AnalyticsEngine::new(),
            insights_engine: InsightsEngine::new(),
            recommendation_engine: RecommendationEngine::new(),
            comfort_score_engine: ComfortScoreEngine::new(),
            application_rule_engine: ApplicationRuleEngine::new(),
        }
    }

    pub fn generate_payload(&self, context: &IntelligenceContext) -> IntelligencePayload {
        // Flow: Context -> Behavior -> Learning -> Analytics -> Insights -> Recommendations -> Score
        
        let behavior = self.behavior_engine.analyze(context);
        let learning = self.learning_engine.extract_observations(context, &behavior);
        let comfort_score = self.comfort_score_engine.calculate(context);
        let analytics = self.analytics_engine.generate(context, comfort_score.total_score, &learning);
        let insights = self.insights_engine.generate(context);
        let recommendations = self.recommendation_engine.generate(context, &behavior);

        IntelligencePayload {
            comfort_score,
            insights,
            recommendations,
            analytics,
        }
    }
}
