use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConstraint {
    pub min_version: String,
    pub max_version: Option<String>,
}

pub struct CompatibilityManager {
    #[allow(dead_code)] // Reserved for future version comparisons
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
        let parse_version = |v: &str| -> Vec<u32> {
            v.split('.')
             .filter_map(|s| s.parse::<u32>().ok())
             .collect()
        };

        let current = parse_version(&self.plugin_sdk_version);
        let min = parse_version(min_sdk);
        
        for (c, m) in current.into_iter().zip(min.into_iter()) {
            if c != m {
                return c > m;
            }
        }
        true // equal or all compared segments equal
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
