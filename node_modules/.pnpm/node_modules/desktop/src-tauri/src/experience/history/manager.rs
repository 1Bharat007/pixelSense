use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use crate::experience::history::models::HistoryEvent;
use crate::experience::history::storage::JsonlStorage;
use crate::experience::history::rotation::RotationStrategy;
use crate::intelligence::models::HistorySummary;

pub struct HistoryManager {
    storage: Arc<Mutex<JsonlStorage>>,
}

impl HistoryManager {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let history_dir = app_data_dir.join("history");
        std::fs::create_dir_all(&history_dir).unwrap_or_default();
        
        let rotation = RotationStrategy::new(history_dir);
        let storage = JsonlStorage::new(rotation);
        
        Self {
            storage: Arc::new(Mutex::new(storage)),
        }
    }

    pub fn record_event(&self, event: HistoryEvent) {
        if let Ok(storage) = self.storage.lock() {
            if let Err(e) = storage.append(event) {
                log::error!("Failed to append history event: {}", e);
            }
        }
    }

    pub fn get_summary(&self) -> HistorySummary {
        // MVP: return an empty/placeholder summary.
        // Full implementation will stream/parse the current JSONL file incrementally.
        HistorySummary {
            total_events: 0,
            brightness_changes_today: 0,
            manual_overrides_today: 0,
            longest_session_minutes: 0,
            average_ambient_lux: 0.0,
        }
    }
}
