use crate::ambient::models::{AmbientQuality, AmbientSensorType, SensorHealth, SensorInfo, SensorState};
use crate::background::models::now_ms;

/// Evaluates the confidence score of an ambient reading.
/// Returns a tuple of (confidence_score, quality, is_stale).
pub struct ConfidenceEvaluator;

impl ConfidenceEvaluator {
    pub fn evaluate(
        info: &SensorInfo,
        health: &SensorHealth,
        sensor_type: &AmbientSensorType,
        is_stable: bool,
        stale_timeout_ms: u64,
        calibration_penalty: f32, // 0.0 means perfect calibration bounds
    ) -> (f32, AmbientQuality, bool) {
        if health.current_state == SensorState::Unavailable || *sensor_type == AmbientSensorType::EstimatedUnavailable {
            return (0.0, AmbientQuality::Poor, false);
        }

        let now = now_ms();
        let age_ms = now.saturating_sub(health.last_update);
        let is_stale = age_ms > stale_timeout_ms;

        // 1. Hardware Quality (30%)
        let hw_score = match sensor_type {
            AmbientSensorType::NativeSensor => 0.30,
            AmbientSensorType::ExternalSensor => 0.25,
            AmbientSensorType::Unknown => 0.10,
            AmbientSensorType::EstimatedUnavailable => 0.0,
        };

        // 2. Reading Freshness (30%)
        // Score decays from 0.30 down to 0.0 linearly as it approaches stale_timeout_ms
        let freshness_ratio = 1.0 - (age_ms as f32 / stale_timeout_ms as f32).clamp(0.0, 1.0);
        let freshness_score = 0.30 * freshness_ratio;

        // 3. Reading Stability (20%)
        let stability_score = if is_stable { 0.20 } else { 0.05 };

        // 4. Sensor Health (10%)
        let health_ratio = if health.total_updates > 0 {
            let success_rate = (health.total_updates.saturating_sub(health.missed_updates)) as f32 / health.total_updates as f32;
            success_rate.clamp(0.0, 1.0)
        } else {
            1.0 // Assume healthy if no history
        };
        let health_score = 0.10 * health_ratio;

        // 5. Calibration Quality (10%)
        let cal_score = 0.10 * (1.0 - calibration_penalty.clamp(0.0, 1.0));

        let mut total_confidence = hw_score + freshness_score + stability_score + health_score + cal_score;
        total_confidence = total_confidence.clamp(0.0, 1.0);

        if is_stale {
            total_confidence *= 0.5; // Stale penalty
        }

        let quality = if is_stale {
            AmbientQuality::Stale
        } else if total_confidence > 0.8 {
            AmbientQuality::Excellent
        } else if total_confidence > 0.5 {
            AmbientQuality::Good
        } else if total_confidence > 0.2 {
            AmbientQuality::Fair
        } else {
            AmbientQuality::Poor
        };

        (total_confidence, quality, is_stale)
    }
}
