use crate::comfort::error::ComfortError;
use crate::comfort::models::ComfortProfile;
use std::fs;
use std::path::PathBuf;

pub trait ComfortStorage: Send + Sync {
    fn save_profile(&self, profile: &ComfortProfile) -> Result<(), ComfortError>;
    fn load_profiles(&self) -> Result<Vec<ComfortProfile>, ComfortError>;
    fn load_profiles_for_display(&self, display_id: &str) -> Result<Vec<ComfortProfile>, ComfortError>;
}

pub struct FileComfortStorage {
    file_path: PathBuf,
}

impl FileComfortStorage {
    pub fn new(file_path: PathBuf) -> Self {
        if !file_path.exists() {
            if let Some(parent) = file_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&file_path, "[]");
        }
        Self { file_path }
    }
}

impl ComfortStorage for FileComfortStorage {
    fn save_profile(&self, profile: &ComfortProfile) -> Result<(), ComfortError> {
        let mut profiles = self.load_profiles().unwrap_or_default();
        
        // Overwrite if exists, else append
        if let Some(existing) = profiles.iter_mut().find(|p| p.profile_id == profile.profile_id) {
            *existing = profile.clone();
        } else {
            profiles.push(profile.clone());
        }

        let json = serde_json::to_string_pretty(&profiles)
            .map_err(|e| ComfortError::StorageFailure(e.to_string()))?;
            
        fs::write(&self.file_path, json)
            .map_err(|e| ComfortError::StorageFailure(e.to_string()))
    }

    fn load_profiles(&self) -> Result<Vec<ComfortProfile>, ComfortError> {
        let data = fs::read_to_string(&self.file_path)
            .map_err(|e| ComfortError::StorageFailure(e.to_string()))?;
            
        let profiles: Vec<ComfortProfile> = serde_json::from_str(&data)
            .map_err(|e| ComfortError::InvalidProfile(e.to_string()))?;
            
        Ok(profiles)
    }

    fn load_profiles_for_display(&self, display_id: &str) -> Result<Vec<ComfortProfile>, ComfortError> {
        let profiles = self.load_profiles()?;
        Ok(profiles.into_iter().filter(|p| p.display_identifier == display_id).collect())
    }
}
