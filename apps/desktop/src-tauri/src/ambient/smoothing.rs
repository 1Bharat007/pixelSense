use std::collections::VecDeque;
use std::sync::Mutex;

pub trait AmbientSmoothingStrategy: Send + Sync {
    fn smooth(&self, new_lux: f32) -> f32;
    fn get_variance(&self) -> f32;
}

pub enum SmoothingStrategyType {
    MovingAverage(usize),
    // Future strategies
    Kalman,
    ExponentialMovingAverage,
    WeightedAverage,
    AdaptiveFilter,
}

pub struct BasicSmoothingStrategy {
    history: Mutex<VecDeque<f32>>,
    max_samples: usize,
}

impl BasicSmoothingStrategy {
    pub fn new(max_samples: usize) -> Self {
        Self {
            history: Mutex::new(VecDeque::with_capacity(max_samples)),
            max_samples,
        }
    }
}

impl AmbientSmoothingStrategy for BasicSmoothingStrategy {
    fn smooth(&self, new_lux: f32) -> f32 {
        let mut history = self.history.lock().unwrap();
        if history.len() >= self.max_samples {
            history.pop_front();
        }
        history.push_back(new_lux);

        let sum: f32 = history.iter().sum();
        let count = history.len() as f32;
        
        sum / count
    }

    fn get_variance(&self) -> f32 {
        let history = self.history.lock().unwrap();
        if history.is_empty() {
            return 0.0;
        }
        let sum: f32 = history.iter().sum();
        let count = history.len() as f32;
        let mean = sum / count;

        let variance_sum: f32 = history.iter().map(|&x| (x - mean) * (x - mean)).sum();
        variance_sum / count
    }
}
