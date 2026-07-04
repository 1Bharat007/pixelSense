use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub title: String,
    pub reason: String,
    pub priority: String, // "Low", "Medium", "High"
    pub estimated_benefit: String,
    pub dismissable: bool,
    pub action: String, // e.g., "ENABLE_NIGHT_MODE", "INCREASE_TRANSITION_TIME"
}
