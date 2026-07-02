use crate::ambient::models::{AmbientQuality, AmbientReading, AmbientSensorState, AmbientSensorType};

pub struct ConfidenceEvaluator;

impl ConfidenceEvaluator {
    pub fn evaluate(
        state: &AmbientSensorState,
        reading: &AmbientReading,
    ) -> (f32, AmbientQuality) {
        if *state != AmbientSensorState::Stable && *state != AmbientSensorState::Reading {
            return (0.0, AmbientQuality::Poor);
        }

        let mut base_confidence = match reading.sensor_type {
            AmbientSensorType::NativeSensor => 1.0,
            AmbientSensorType::ExternalSensor => 0.9,
            AmbientSensorType::Estimated => 0.5,
            AmbientSensorType::Unknown => 0.2,
        };

        if !reading.is_stable {
            base_confidence *= 0.5;
        }

        let quality = if base_confidence > 0.8 {
            AmbientQuality::Excellent
        } else if base_confidence > 0.5 {
            AmbientQuality::Good
        } else if base_confidence > 0.2 {
            AmbientQuality::Fair
        } else {
            AmbientQuality::Poor
        };

        (base_confidence, quality)
    }
}
