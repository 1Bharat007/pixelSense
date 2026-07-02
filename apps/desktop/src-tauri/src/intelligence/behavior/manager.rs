use crate::intelligence::models::IntelligenceContext;
use crate::intelligence::behavior::models::UserBehaviorSnapshot;

pub struct BehaviorEngine;

impl BehaviorEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, context: &IntelligenceContext) -> UserBehaviorSnapshot {
        // In a full implementation, this parses rolling history.
        // For MVP, we map context and history summary deterministically.

        let overrides = context.history_summary.manual_overrides_today;
        let preferred_range = if context.current_ambient_lux < 50.0 {
            (10, 40)
        } else {
            (50, 90)
        };

        UserBehaviorSnapshot {
            active_application: context.active_application.clone(),
            session_duration_minutes: context.history_summary.longest_session_minutes,
            manual_overrides_today: overrides,
            ignored_recommendations: 0, // Placeholder
            profile_switches_today: 1,  // Placeholder
            fullscreen_usage_minutes: 0,
            power_mode: context.performance_policy.clone(),
            monitor_usage_count: 1, // Single monitor fallback
            average_transition_speed: "Balanced".into(),
            preferred_brightness_range: preferred_range,
        }
    }
}
