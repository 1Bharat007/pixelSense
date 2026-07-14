use std::sync::Mutex;
use crate::logging::models::LogEntry;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct LogManager {
    // In production, this would use a proper rolling file appender and non-blocking queue.
    file: Mutex<Option<File>>,
    session_id: String,
}

impl LogManager {
    pub fn new(session_id: String) -> Self {
        Self {
            file: Mutex::new(None),
            session_id,
        }
    }

    pub fn initialize(&self, log_path: &str) -> Result<(), String> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .map_err(|e| format!("Failed to open log file: {}", e))?;
            
        *self.file.lock().unwrap() = Some(file);
        Ok(())
    }

    pub fn log(&self, entry: LogEntry) {
        if let Ok(json) = serde_json::to_string(&entry) {
            let mut lock = self.file.lock().unwrap();
            if let Some(ref mut file) = *lock {
                let _ = writeln!(file, "{}", json);
            } else {
                // Fallback if not initialized (though we shouldn't use println! in prod, 
                // this is just for the structural scaffold if file fails).
                // println!("{}", json);
            }
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}
