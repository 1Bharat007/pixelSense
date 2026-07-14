use crate::ambient::calibration::CalibrationStrategy;
use crate::ambient::config::AmbientConfig;
use crate::ambient::confidence::ConfidenceEvaluator;
use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientDiagnostics, AmbientEnvironment, AmbientQuality, AmbientReading, AmbientSensorType, SensorHealth, SensorState};
use crate::ambient::registry::SensorRegistry;
use crate::ambient::smoothing::AmbientSmoothingStrategy;
use crate::background::models::now_ms;
use std::sync::Mutex;

pub struct AmbientManager {
    config: AmbientConfig,
    registry: SensorRegistry,
    calibration: Box<dyn CalibrationStrategy>,
    smoothing: Box<dyn AmbientSmoothingStrategy>,
    
    // Health and state tracking
    health: Mutex<SensorHealth>,
    last_smoothed_lux: Mutex<Option<f32>>,
    diagnostics: Mutex<AmbientDiagnostics>,
}

impl AmbientManager {
    pub fn new(
        config: AmbientConfig,
        registry: SensorRegistry,
        calibration: Box<dyn CalibrationStrategy>,
        smoothing: Box<dyn AmbientSmoothingStrategy>,
    ) -> Self {
        Self {
            config,
            registry,
            calibration,
            smoothing,
            health: Mutex::new(SensorHealth {
                last_update: 0,
                update_frequency_ms: 0,
                total_updates: 0,
                missed_updates: 0,
                failure_count: 0,
                recovery_count: 0,
                current_state: SensorState::Discovering,
            }),
            last_smoothed_lux: Mutex::new(None),
            diagnostics: Mutex::new(AmbientDiagnostics {
                sensor_available: false,
                provider: "None".into(),
                confidence: 0.0,
                last_read: 0,
                poll_count: 0,
                failure_count: 0,
                last_error: None,
                callback_active: false,
                cached_reading_age_ms: 0,
                sensor_state: SensorState::Discovering,
                sensor_count: 0,
                stale_reading: false,
                last_callback_duration_ms: 0,
            }),
        }
    }

    /// Read ambient light safely.
    /// In the event of no sensor, returns the Fallback Reading (never fails).
    pub fn get_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        let now = now_ms();
        let mut diag = self.diagnostics.lock().unwrap();
        diag.poll_count += 1;
        
        let infos = self.registry.get_infos();
        diag.sensor_count = infos.len();

        let raw_reading = match self.registry.read_primary() {
            Ok(r) => r,
            Err(_) => {
                // Fallback Policy: Never panic, never fail the background worker.
                let mut health = self.health.lock().unwrap();
                health.current_state = SensorState::Unavailable;
                diag.sensor_available = false;
                diag.sensor_state = SensorState::Unavailable;
                
                let fallback = AmbientReading {
                    source_id: "fallback".into(),
                    sensor_name: "Fallback".into(),
                    lux: 0.0,
                    normalized_lux: 0.0,
                    environment: AmbientEnvironment::Unknown,
                    confidence: 0.0,
                    sensor_type: AmbientSensorType::EstimatedUnavailable,
                    timestamp: now,
                    quality: AmbientQuality::Poor,
                    is_stable: true,
                    reading_duration_ms: 0,
                    is_estimated: true,
                };
                return Ok(fallback);
            }
        };

        // Calibration
        let calibrated_lux = self.calibration.calibrate(raw_reading.lux);

        // Smoothing
        let smoothed_lux = if self.config.smoothing_enabled {
            self.smoothing.smooth(calibrated_lux)
        } else {
            calibrated_lux
        };

        let mut last_smoothed = self.last_smoothed_lux.lock().unwrap();
        let is_stable = if let Some(prev) = *last_smoothed {
            (smoothed_lux - prev).abs() < self.config.minimum_change_threshold
        } else {
            true
        };
        
        let final_lux = if is_stable && last_smoothed.is_some() {
            last_smoothed.unwrap()
        } else {
            smoothed_lux
        };
        *last_smoothed = Some(final_lux);

        let mut health = self.health.lock().unwrap();
        
        // Ensure state is Available
        if health.current_state != SensorState::Available {
            health.current_state = SensorState::Available;
            health.recovery_count += 1;
        }

        // Update health tracking if new timestamp
        if raw_reading.timestamp > health.last_update {
            if health.last_update > 0 {
                health.update_frequency_ms = raw_reading.timestamp.saturating_sub(health.last_update);
            }
            health.last_update = raw_reading.timestamp;
            health.total_updates += 1;
        }

        // We assume primary sensor is infos[0] for evaluation
        let info = infos.first().unwrap();

        // Calculate Confidence
        let (confidence, quality, is_stale) = ConfidenceEvaluator::evaluate(
            info,
            &health,
            &raw_reading.sensor_type,
            is_stable,
            self.config.stale_timeout_ms,
            0.0, // calibration penalty (placeholder)
        );

        // Update diagnostics
        diag.sensor_available = true;
        diag.provider = raw_reading.source_id.clone();
        diag.confidence = confidence;
        diag.last_read = now;
        diag.sensor_state = health.current_state.clone();
        diag.stale_reading = is_stale;
        diag.cached_reading_age_ms = now.saturating_sub(raw_reading.timestamp);
        diag.callback_active = info.supports_events && !is_stale;

        let mut final_reading = raw_reading.clone();
        final_reading.lux = calibrated_lux;
        final_reading.normalized_lux = final_lux;
        final_reading.environment = AmbientReading::from_lux(final_lux);
        final_reading.confidence = confidence;
        final_reading.quality = quality;
        final_reading.is_stable = is_stable;

        Ok(final_reading)
    }

    pub fn get_diagnostics(&self) -> AmbientDiagnostics {
        self.diagnostics.lock().unwrap().clone()
    }
    
    pub fn get_health(&self) -> SensorHealth {
        self.health.lock().unwrap().clone()
    }

    pub fn suspend(&self) {
        let mut health = self.health.lock().unwrap();
        health.current_state = SensorState::Sleeping;
        self.registry.suspend_all();
    }

    pub fn resume(&self) {
        let mut health = self.health.lock().unwrap();
        health.current_state = SensorState::Discovering;
        self.registry.resume_all();
    }
}
