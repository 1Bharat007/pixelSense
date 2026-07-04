pub struct SecurityLimits;

impl SecurityLimits {
    pub const MAX_JSON_FILE_SIZE_BYTES: usize = 5 * 1024 * 1024; // 5MB limit
    pub const MAX_LOG_FILE_SIZE_BYTES: usize = 50 * 1024 * 1024; // 50MB limit
    pub const MAX_HISTORY_EVENT_SIZE_BYTES: usize = 256 * 1024; // 256KB
    pub const MAX_PLUGIN_MANIFEST_SIZE_BYTES: usize = 64 * 1024; // 64KB
    pub const MAX_CONFIGURATION_DEPTH: usize = 16;
    
    pub fn validate_file_size(size: usize, limit: usize) -> Result<(), String> {
        if size > limit {
            Err(format!("File size {} exceeds limit {}", size, limit))
        } else {
            Ok(())
        }
    }
}
