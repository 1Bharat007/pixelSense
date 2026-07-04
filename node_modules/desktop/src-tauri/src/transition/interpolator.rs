use crate::transition::config::TransitionConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionStep {
    pub brightness: u8,
    pub delay_ms: u64,
}

pub struct LinearInterpolator;

impl LinearInterpolator {
    /// Calculates the intermediate brightness steps.
    pub fn interpolate(
        current: u8,
        target: u8,
        duration_ms: u64,
        config: &TransitionConfig,
    ) -> Vec<TransitionStep> {
        if duration_ms == 0 || current == target {
            return vec![TransitionStep {
                brightness: target,
                delay_ms: 0,
            }];
        }

        let ticks = (duration_ms / config.tick_interval_ms).max(1);
        let mut steps = Vec::with_capacity(ticks as usize);

        let diff = (target as f32) - (current as f32);
        let step_amount = diff / (ticks as f32);

        for i in 1..=ticks {
            let val = (current as f32 + (step_amount * i as f32)).round() as u8;
            steps.push(TransitionStep {
                brightness: val.clamp(0, 100),
                delay_ms: config.tick_interval_ms,
            });
        }

        // Ensure the final step is exactly the target
        if let Some(last) = steps.last_mut() {
            last.brightness = target;
        }

        steps
    }
}
