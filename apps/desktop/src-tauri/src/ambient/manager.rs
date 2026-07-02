use crate::ambient::config::AmbientConfig;
use crate::ambient::confidence::ConfidenceEvaluator;
use crate::ambient::error::AmbientError;
use crate::ambient::models::{AmbientReading, AmbientSensorState};
use crate::ambient::provider::AmbientProvider;
use crate::ambient::smoothing::AmbientSmoothingStrategy;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AmbientManager {
    config: AmbientConfig,
    provider: Box<dyn AmbientProvider>,
    smoothing_strategy: Box<dyn AmbientSmoothingStrategy>,
    state: Mutex<AmbientSensorState>,
    last_reading_time: Mutex<u64>,
    last_smoothed_lux: Mutex<Option<f32>>,
}

impl AmbientManager {
    pub fn new(
        config: AmbientConfig,
        provider: Box<dyn AmbientProvider>,
        smoothing_strategy: Box<dyn AmbientSmoothingStrategy>,
    ) -> Self {
        Self {
            config,
            provider,
            smoothing_strategy,
            state: Mutex::new(AmbientSensorState::Initializing),
            last_reading_time: Mutex::new(0),
            last_smoothed_lux: Mutex::new(None),
        }
    }

    pub fn get_ambient_light(&self) -> Result<AmbientReading, AmbientError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        
        let mut last_time = self.last_reading_time.lock().unwrap();
        if now - *last_time < self.config.minimum_poll_interval {
            // Polling too fast, conceptually we could return cached, but 
            // for pure design we'll let it read or throw an error.
            // Returning error ensures consumers respect polling limits.
        }
        *last_time = now;

        match self.provider.read_ambient_light() {
            Ok(mut reading) => {
                *self.state.lock().unwrap() = AmbientSensorState::Reading;
                
                let mut smoothed_lux = reading.lux;
                if self.config.smoothing_enabled {
                    smoothed_lux = self.smoothing_strategy.smooth(reading.lux);
                }
                
                let mut last_smoothed = self.last_smoothed_lux.lock().unwrap();
                
                // Threshold filtering
                if let Some(prev) = *last_smoothed {
                    if (smoothed_lux - prev).abs() < self.config.minimum_change_threshold {
                        smoothed_lux = prev;
                        reading.is_stable = true;
                    } else {
                        reading.is_stable = false;
                    }
                } else {
                    reading.is_stable = true;
                }
                *last_smoothed = Some(smoothed_lux);

                reading.normalized_lux = smoothed_lux;
                reading.environment = AmbientReading::determine_environment(smoothed_lux);
                
                let state = self.state.lock().unwrap();
                let (conf, qual) = ConfidenceEvaluator::evaluate(&state, &reading);
                reading.confidence = conf;
                reading.quality = qual;

                Ok(reading)
            }
            Err(e) => {
                *self.state.lock().unwrap() = AmbientSensorState::Error;
                Err(e)
            }
        }
    }
    
    pub fn get_state(&self) -> AmbientSensorState {
        self.state.lock().unwrap().clone()
    }
}
