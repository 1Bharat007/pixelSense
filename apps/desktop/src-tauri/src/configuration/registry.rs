use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSchema {
    pub key: String,
    pub default_value: String,
    pub description: String,
    pub requires_restart: bool,
}

pub struct ConfigurationRegistry {
    schemas: RwLock<HashMap<String, ConfigurationSchema>>,
    values: RwLock<HashMap<String, String>>,
}

impl ConfigurationRegistry {
    pub fn new() -> Self {
        Self {
            schemas: RwLock::new(HashMap::new()),
            values: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_schema(&self, schema: ConfigurationSchema) {
        let mut schemas = self.schemas.write().unwrap();
        schemas.insert(schema.key.clone(), schema);
    }

    pub fn set_value(&self, key: &str, value: String) -> Result<(), String> {
        let schemas = self.schemas.read().unwrap();
        if !schemas.contains_key(key) {
            return Err(format!("Configuration key {} not registered", key));
        }
        
        let mut values = self.values.write().unwrap();
        values.insert(key.to_string(), value);
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        let values = self.values.read().unwrap();
        if let Some(val) = values.get(key) {
            return Some(val.clone());
        }
        
        let schemas = self.schemas.read().unwrap();
        schemas.get(key).map(|s| s.default_value.clone())
    }
}
