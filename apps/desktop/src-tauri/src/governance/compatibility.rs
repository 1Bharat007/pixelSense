use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConstraint {
    pub min_version: String,
    pub max_version: Option<String>,
}

pub struct CompatibilityManager {
    app_version: String,
    plugin_sdk_version: String,
}

impl CompatibilityManager {
    pub fn new(app_version: &str, plugin_sdk_version: &str) -> Self {
        Self {
            app_version: app_version.to_string(),
            plugin_sdk_version: plugin_sdk_version.to_string(),
        }
    }

    pub fn is_plugin_compatible(&self, min_sdk: &str) -> bool {
        // Simplified semver check for architecture blueprint
        self.plugin_sdk_version >= min_sdk.to_string()
    }

    pub fn validate_schema_version(&self, current_schema: u32, expected_schema: u32) -> Result<(), String> {
        if current_schema != expected_schema {
            return Err(format!(
                "Schema mismatch: expected {}, got {}",
                expected_schema, current_schema
            ));
        }
        Ok(())
    }
}
