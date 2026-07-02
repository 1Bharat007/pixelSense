use std::collections::VecDeque;
use std::sync::Mutex;

pub trait AmbientSmoothingStrategy: Send + Sync {
    fn smooth(&self, new_lux: f32) -> f32;
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
}
