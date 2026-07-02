use crate::comfort::manager::ComfortManager;
use crate::comfort::storage::FileComfortStorage;
use crate::comfort::strategies::nearest_neighbor::NearestNeighborStrategy;
use std::path::PathBuf;

pub fn create_comfort_manager(config_dir: PathBuf) -> ComfortManager {
    let storage_path = config_dir.join("profiles.json");
    let storage = Box::new(FileComfortStorage::new(storage_path));
    let matching_strategy = Box::new(NearestNeighborStrategy::new());
    
    ComfortManager::new(storage, matching_strategy)
}
