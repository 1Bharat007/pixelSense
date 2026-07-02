use crate::comfort::models::ComfortProfile;

#[derive(Debug, Clone)]
pub struct VisualComfortContext {
    pub display_id: String,
    pub current_comfort_profile: Option<ComfortProfile>,
    pub ambient_light: Option<f32>,
    pub screen_luminance: Option<f32>,
    pub current_monitor_brightness: u8,
    pub predicted_emitted_light: f32,
    pub time_of_day: String, // E.g., "Day", "Night"
    pub transition_enabled: bool,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct ComfortConfig {
    pub minimum_change_threshold: u8,
    pub minimum_update_interval: u64, // ms
    pub maximum_step_change: u8,
    pub minimum_brightness: u8,
    pub maximum_brightness: u8,
    pub preferred_transition_duration: u64, // ms
    pub stabilization_enabled: bool,
    pub emergency_mode_enabled: bool,
}

impl Default for ComfortConfig {
    fn default() -> Self {
        Self {
            minimum_change_threshold: 3,
            minimum_update_interval: 1000,
            maximum_step_change: 20,
            minimum_brightness: 10,
            maximum_brightness: 100,
            preferred_transition_duration: 500,
            stabilization_enabled: true,
            emergency_mode_enabled: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationAction {
    NoChange,
    SmoothTransition,
    ImmediateTransition,
    Ignore,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComfortRecommendation {
    pub recommended_brightness: u8,
    pub confidence: f32,
    pub reason: String,
    pub action: RecommendationAction,
}

#[derive(Debug, Clone)]
pub struct VisualComfortResult {
    pub recommendation: ComfortRecommendation,
    pub comfort_delta: f32,
    pub predicted_eye_comfort: f32,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComfortState {
    Stable,
    Adjusting,
    CoolingDown,
    WaitingForTransition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VisualComfortError {
    NoProfileFound(String),
    UnsupportedDisplay(String),
    LowConfidence(String),
    InvalidContext(String),
}

impl std::fmt::Display for VisualComfortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoProfileFound(m) => write!(f, "No profile found: {}", m),
            Self::UnsupportedDisplay(m) => write!(f, "Unsupported display: {}", m),
            Self::LowConfidence(m) => write!(f, "Low confidence: {}", m),
            Self::InvalidContext(m) => write!(f, "Invalid context: {}", m),
        }
    }
}
impl std::error::Error for VisualComfortError {}
