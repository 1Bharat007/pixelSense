use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerManifest {
    pub version: String,
    pub platform: String,
    pub arch: String,
    pub required_disk_space_mb: u32,
    pub min_os_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallationMode {
    PerUser,
    PerMachine,
    Portable,
    Silent,
}

#[derive(Debug, Clone)]
pub struct InstallValidationResult {
    pub is_compatible: bool,
    pub has_permissions: bool,
    pub space_available: bool,
    pub error_message: Option<String>,
}
