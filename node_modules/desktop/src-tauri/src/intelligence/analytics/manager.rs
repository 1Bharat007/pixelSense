use crate::intelligence::models::IntelligenceContext;
use crate::intelligence::analytics::models::{
    AnalyticsSnapshot, DailyAnalytics, MonthlyAnalytics, RealtimeAnalytics, WeeklyAnalytics,
};
use crate::intelligence::learning::models::LearningObservation;

pub struct AnalyticsEngine;

impl AnalyticsEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(
        &self,
        context: &IntelligenceContext,
        score: u8,
        _observations: &[LearningObservation],
    ) -> AnalyticsSnapshot {
        // MVP: Map directly from context. In future, aggregates rolling history.
        let realtime = RealtimeAnalytics {
            current_comfort_score: score,
            active_monitors: 1, // Fallback
        };

        let daily = DailyAnalytics {
            average_comfort_score: score, // MVP approximation
            total_transitions: context.history_summary.brightness_changes_today,
            manual_overrides: context.history_summary.manual_overrides_today,
            average_ambient_lux: context.history_summary.average_ambient_lux,
            average_screen_luminance: context.current_screen_luminance,
        };

        let weekly = WeeklyAnalytics {
            average_comfort_score: score,
            active_days: 1,
        };

        let monthly = MonthlyAnalytics {
            average_comfort_score: score,
        };

        AnalyticsSnapshot {
            realtime,
            daily,
            weekly,
            monthly,
        }
    }
}
