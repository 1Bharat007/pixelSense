use crate::visual_comfort::models::ComfortConfig;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait RateLimiter: Send + Sync {
    fn should_update(&self, display_id: &str, current_brightness: u8, target_brightness: u8, config: &ComfortConfig) -> bool;
    fn record_update(&mut self, display_id: &str);
}

pub struct DefaultRateLimiter {
    // In reality, map of display_id to last update timestamp
    last_update_ms: u64,
}

impl DefaultRateLimiter {
    pub fn new() -> Self {
        Self { last_update_ms: 0 }
    }
}

impl RateLimiter for DefaultRateLimiter {
    fn should_update(&self, _display_id: &str, current_brightness: u8, target_brightness: u8, config: &ComfortConfig) -> bool {
        let diff = (current_brightness as i16 - target_brightness as i16).abs() as u8;
        if diff < config.minimum_change_threshold {
            return false;
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        if now - self.last_update_ms < config.minimum_update_interval {
            return false;
        }

        true
    }

    fn record_update(&mut self, _display_id: &str) {
        self.last_update_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    }
}
