use std::path::PathBuf;
use chrono::Local;

pub struct RotationStrategy {
    base_dir: PathBuf,
}

impl RotationStrategy {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Returns the active JSONL file path for the current day.
    pub fn get_current_file_path(&self) -> PathBuf {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let filename = format!("history_{}.jsonl", today);
        self.base_dir.join(filename)
    }
}
