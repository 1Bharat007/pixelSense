use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub title: String,
    pub description: String,
    pub severity: String, // "Low", "Medium", "High"
    pub category: String, // "Comfort", "Environment", "Performance"
    pub confidence: f32,  // 0.0 to 1.0
    pub icon: String,     // e.g., "Sun", "Moon", "Eye"
    pub timestamp: u64,
    pub related_events: Vec<String>,
}
