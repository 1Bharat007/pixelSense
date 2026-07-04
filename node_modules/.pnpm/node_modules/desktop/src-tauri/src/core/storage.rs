use std::path::PathBuf;
use std::fs;
use std::io::Write;

pub struct StorageManager {
    base_path: PathBuf,
}

impl StorageManager {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// Performs an atomic write by writing to a temporary file and renaming it.
    pub fn write_atomic(&self, relative_path: &str, content: &[u8]) -> Result<(), String> {
        let mut target_path = self.base_path.clone();
        target_path.push(relative_path);

        let mut temp_path = target_path.clone();
        temp_path.set_extension("tmp");

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let mut file = fs::File::create(&temp_path).map_err(|e| e.to_string())?;
        file.write_all(content).map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;

        fs::rename(&temp_path, &target_path).map_err(|e| {
            let _ = fs::remove_file(&temp_path);
            e.to_string()
        })?;

        Ok(())
    }

    pub fn read(&self, relative_path: &str) -> Result<Vec<u8>, String> {
        let mut target_path = self.base_path.clone();
        target_path.push(relative_path);
        
        fs::read(&target_path).map_err(|e| e.to_string())
    }
}
