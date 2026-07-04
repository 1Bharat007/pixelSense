use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct ServiceRegistry {
    services: RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<T: Any + Send + Sync>(&self, id: &str, service: Arc<T>) {
        let mut services = self.services.write().unwrap();
        services.insert(id.to_string(), service);
    }

    pub fn resolve<T: Any + Send + Sync>(&self, id: &str) -> Option<Arc<T>> {
        let services = self.services.read().unwrap();
        if let Some(service_any) = services.get(id) {
            // Downcast the trait object to the concrete type
            return service_any.clone().downcast::<T>().ok();
        }
        None
    }
}
