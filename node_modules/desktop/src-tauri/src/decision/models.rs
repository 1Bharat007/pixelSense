use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionResult {
    pub recommended_brightness: u8,
    pub confidence: f32, // 0.0 to 1.0
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AmbientLightReading {
    pub lux: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComfortLevel {
    VeryDim,
    Dim,
    Balanced,
    Bright,
    VeryBright,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeOfDay {
    Morning,
    Day,
    Evening,
    Night,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionContext {
    pub ambient_light: Option<AmbientLightReading>,
    pub user_brightness_preference: Option<u8>,
    pub comfort_preference: ComfortLevel,
    pub time_of_day: TimeOfDay,
}
