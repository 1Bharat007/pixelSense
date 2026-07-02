pub mod error;
pub mod factory;
pub mod manager;
pub mod models;
pub mod storage;
pub mod strategies;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use crate::comfort::factory::create_comfort_manager;
    use crate::comfort::error::ComfortError;

    fn get_test_dir() -> PathBuf {
        let dir = PathBuf::from("test_profiles_dir");
        let _ = fs::create_dir_all(&dir);
        dir
    }

    fn cleanup_test_dir(dir: &PathBuf) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_save_and_load_profile() {
        let dir = get_test_dir();
        let manager = create_comfort_manager(dir.clone());

        // Lock a profile
        let profile = manager.lock_comfort(
            "disp_1".into(),
            500.0,
            80.0,
            60,
            Some("Office".into()),
        ).unwrap();

        assert_eq!(profile.profile_name, "Office");
        assert_eq!(profile.monitor_brightness, 60);

        // Recommend profile (should match the exact one)
        let result = manager.recommend_comfort("disp_1", 500.0, 80.0).unwrap();
        
        assert_eq!(result.matched_profile.profile_id, profile.profile_id);
        assert_eq!(result.distance, 0.0);
        assert_eq!(result.similarity_score, 1.0);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_nearest_neighbor_matching() {
        let dir = get_test_dir();
        let manager = create_comfort_manager(dir.clone());

        // Save a dark profile
        manager.lock_comfort("disp_1".into(), 10.0, 10.0, 20, None).unwrap();
        
        // Save a bright profile
        manager.lock_comfort("disp_1".into(), 1000.0, 90.0, 100, None).unwrap();

        // Query with something near dark
        let result = manager.recommend_comfort("disp_1", 15.0, 15.0).unwrap();
        assert_eq!(result.matched_profile.monitor_brightness, 20); // Should pick the dark one

        // Query with something near bright
        let result2 = manager.recommend_comfort("disp_1", 900.0, 85.0).unwrap();
        assert_eq!(result2.matched_profile.monitor_brightness, 100); // Should pick the bright one

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_multiple_displays() {
        let dir = get_test_dir();
        let manager = create_comfort_manager(dir.clone());

        manager.lock_comfort("disp_A".into(), 500.0, 50.0, 40, None).unwrap();
        manager.lock_comfort("disp_B".into(), 500.0, 50.0, 70, None).unwrap();

        // Same environment, different displays should yield different profiles
        let result_a = manager.recommend_comfort("disp_A", 500.0, 50.0).unwrap();
        assert_eq!(result_a.matched_profile.monitor_brightness, 40);

        let result_b = manager.recommend_comfort("disp_B", 500.0, 50.0).unwrap();
        assert_eq!(result_b.matched_profile.monitor_brightness, 70);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_no_profiles_found() {
        let dir = get_test_dir();
        let manager = create_comfort_manager(dir.clone());

        let result = manager.recommend_comfort("disp_unknown", 100.0, 50.0);
        assert!(matches!(result, Err(ComfortError::ProfileNotFound(_))));

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_invalid_profile_json() {
        let dir = get_test_dir();
        let file_path = dir.join("profiles.json");
        fs::write(&file_path, "invalid json format").unwrap();

        let manager = create_comfort_manager(dir.clone());
        let result = manager.recommend_comfort("disp_1", 100.0, 50.0);
        
        assert!(matches!(result, Err(ComfortError::InvalidProfile(_))));

        cleanup_test_dir(&dir);
    }
}
