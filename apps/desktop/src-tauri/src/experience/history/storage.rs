use std::fs::OpenOptions;
use std::io::Write;
use crate::experience::history::models::{HistoryEvent, TimestampedEvent};
use crate::experience::history::rotation::RotationStrategy;

pub struct JsonlStorage {
    rotation: RotationStrategy,
}

impl JsonlStorage {
    pub fn new(rotation: RotationStrategy) -> Self {
        Self { rotation }
    }

    pub fn append(&self, event: HistoryEvent) -> Result<(), String> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let timestamped_event = TimestampedEvent { timestamp, event };

        let json = serde_json::to_string(&timestamped_event)
            .map_err(|e| format!("Failed to serialize event: {}", e))?;

        let current_file = self.rotation.get_current_file_path();
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&current_file)
            .map_err(|e| format!("Failed to open storage file: {}", e))?;

        writeln!(file, "{}", json)
            .map_err(|e| format!("Failed to write to storage file: {}", e))?;

        Ok(())
    }
}
