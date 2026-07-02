#[derive(Debug, Clone, PartialEq)]
pub enum AmbientEnvironment {
    PitchBlack,
    DarkRoom,
    DimRoom,
    Indoor,
    BrightOffice,
    OutdoorShade,
    BrightOutdoor,
    DirectSunlight,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmbientSensorType {
    NativeSensor,
    Estimated,
    ExternalSensor,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmbientQuality {
    Excellent,
    Good,
    Fair,
    Poor,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmbientSensorState {
    Unavailable,
    Initializing,
    Reading,
    Stable,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AmbientReading {
    pub source_id: String,
    pub lux: f32,
    pub normalized_lux: f32,
    pub environment: AmbientEnvironment,
    pub confidence: f32,
    pub sensor_type: AmbientSensorType,
    pub timestamp: u64,
    pub quality: AmbientQuality,
    pub is_stable: bool,
}

impl AmbientReading {
    pub fn determine_environment(lux: f32) -> AmbientEnvironment {
        if lux < 1.0 {
            AmbientEnvironment::PitchBlack
        } else if lux < 20.0 {
            AmbientEnvironment::DarkRoom
        } else if lux < 50.0 {
            AmbientEnvironment::DimRoom
        } else if lux < 200.0 {
            AmbientEnvironment::Indoor
        } else if lux < 500.0 {
            AmbientEnvironment::BrightOffice
        } else if lux < 2000.0 {
            AmbientEnvironment::OutdoorShade
        } else if lux < 10000.0 {
            AmbientEnvironment::BrightOutdoor
        } else {
            AmbientEnvironment::DirectSunlight
        }
    }
}
