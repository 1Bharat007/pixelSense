use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningObservation {
    Pattern {
        context: String,
        behavior: String,
    },
    Anomaly {
        description: String,
        severity: String,
    },
    Trend {
        metric: String,
        direction: String,
    },
    Habit {
        trigger: String,
        action: String,
    },
}
