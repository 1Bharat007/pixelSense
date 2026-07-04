use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateChannel {
    Development,
    Nightly,
    Experimental,
    Beta,
    Stable,
    Enterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManifest {
    pub version: String,
    pub release_notes: String,
    pub checksum: String,
    pub download_url: String,
    pub channel: UpdateChannel,
}

#[derive(Debug, Clone)]
pub enum UpdateState {
    Idle,
    Checking,
    UpdateAvailable(UpdateManifest),
    Downloading,
    Validating,
    ReadyToInstall,
    Installing,
    RollbackInitiated,
    Error(String),
}
