use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmbientEnvironment {
    PitchBlack,
    Dark,
    Dim,
    Indoor,
    Office,
    Bright,
    Outdoor,
    DirectSunlight,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmbientSensorType {
    NativeSensor,
    EstimatedUnavailable, // Specifically for our fallback policy
    ExternalSensor,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmbientQuality {
    Excellent,
    Good,
    Fair,
    Poor,
    Stale, // Represents reading that hasn't been updated recently
}

#[derive(Debug, Clone, PartialEq)]
pub enum SensorState {
    Discovering,
    Available,
    Unavailable,
    Sleeping,
    Recovering,
    Error,
    Disposed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SensorInfo {
    pub manufacturer: String,
    pub device_name: String,
    pub hardware_id: String,
    pub driver_version: String,
    // Capabilities
    pub supports_events: bool,
    pub supports_polling: bool,
    pub minimum_lux: f32,
    pub maximum_lux: f32,
    pub sampling_frequency: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SensorHealth {
    pub last_update: u64,
    pub update_frequency_ms: u64,
    pub total_updates: u64,
    pub missed_updates: u64,
    pub failure_count: u32,
    pub recovery_count: u32,
    pub current_state: SensorState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AmbientDiagnostics {
    pub sensor_available: bool,
    pub provider: String,
    pub confidence: f32,
    pub last_read: u64,
    pub poll_count: u64,
    pub failure_count: u32,
    pub last_error: Option<String>,
    // Extended fields
    pub callback_active: bool,
    pub cached_reading_age_ms: u64,
    pub sensor_state: SensorState,
    pub sensor_count: usize,
    pub stale_reading: bool,
    pub last_callback_duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AmbientReading {
    pub source_id: String,
    pub sensor_name: String,
    pub lux: f32,
    pub normalized_lux: f32,
    pub environment: AmbientEnvironment,
    pub confidence: f32,
    pub sensor_type: AmbientSensorType,
    pub timestamp: u64,
    pub quality: AmbientQuality,
    pub is_stable: bool,
    pub reading_duration_ms: u64,
    pub is_estimated: bool,
}

impl AmbientReading {
    pub fn from_lux(lux: f32) -> AmbientEnvironment {
        if lux < 1.0 {
            AmbientEnvironment::PitchBlack
        } else if lux < 10.0 {
            AmbientEnvironment::Dark
        } else if lux < 50.0 {
            AmbientEnvironment::Dim
        } else if lux < 250.0 {
            AmbientEnvironment::Indoor
        } else if lux < 1000.0 {
            AmbientEnvironment::Office
        } else if lux < 5000.0 {
            AmbientEnvironment::Bright
        } else if lux < 20000.0 {
            AmbientEnvironment::Outdoor
        } else {
            AmbientEnvironment::DirectSunlight
        }
    }
}
