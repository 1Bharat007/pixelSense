pub mod linear;

pub trait CalibrationStrategy: Send + Sync {
    /// Normalizes raw lux from the sensor into the PixelSense internal scale.
    /// Also responsible for clamping impossible values.
    fn calibrate(&self, raw_lux: f32) -> f32;
}
