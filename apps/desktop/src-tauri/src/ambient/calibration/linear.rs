use crate::ambient::calibration::CalibrationStrategy;

pub struct LinearCalibration {
    pub max_lux: f32,
}

impl LinearCalibration {
    pub fn new(max_lux: f32) -> Self {
        Self { max_lux }
    }
}

impl Default for LinearCalibration {
    fn default() -> Self {
        Self::new(10_000.0) // Direct sunlight baseline max
    }
}

impl CalibrationStrategy for LinearCalibration {
    fn calibrate(&self, raw_lux: f32) -> f32 {
        if raw_lux <= 0.0 {
            return 0.0;
        }
        let clamped = raw_lux.min(self.max_lux);
        clamped
    }
}
