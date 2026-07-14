use crate::visual_comfort::models::ComfortConfig;
use std::sync::Mutex;

pub trait ComfortStabilizer: Send + Sync {
    fn stabilize(&self, raw_luminance: f32, config: &ComfortConfig) -> f32;
}

pub struct DefaultComfortStabilizer {
    last_luminance: Mutex<Option<f32>>,
}

impl DefaultComfortStabilizer {
    pub fn new() -> Self {
        Self {
            last_luminance: Mutex::new(None),
        }
    }
}

impl ComfortStabilizer for DefaultComfortStabilizer {
    fn stabilize(&self, raw_luminance: f32, config: &ComfortConfig) -> f32 {
        if !config.stabilization_enabled {
            return raw_luminance;
        }
        
        let mut last_lum = self.last_luminance.lock().unwrap();
        if let Some(prev) = *last_lum {
            // Simple Exponential Weighted Moving Average (EWMA)
            // Alpha determines smoothing factor. Alpha = 0.2 means 20% new, 80% old.
            let alpha = 0.2_f32; 
            let smoothed = prev * (1.0 - alpha) + raw_luminance * alpha;
            *last_lum = Some(smoothed);
            smoothed
        } else {
            *last_lum = Some(raw_luminance);
            raw_luminance
        }
    }
}
