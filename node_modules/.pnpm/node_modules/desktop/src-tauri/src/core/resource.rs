use std::collections::HashMap;
use std::sync::RwLock;

pub struct ResourceManager {
    assets: RwLock<HashMap<String, Vec<u8>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            assets: RwLock::new(HashMap::new()),
        }
    }

    pub fn load_asset(&self, id: &str, content: Vec<u8>) {
        let mut assets = self.assets.write().unwrap();
        assets.insert(id.to_string(), content);
    }

    pub fn get_asset(&self, id: &str) -> Option<Vec<u8>> {
        let assets = self.assets.read().unwrap();
        assets.get(id).cloned()
    }
}
