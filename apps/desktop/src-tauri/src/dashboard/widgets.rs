use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait WidgetProvider: Send + Sync {
    fn get_widget_json(&self) -> String;
}

pub struct WidgetRegistry {
    providers: RwLock<HashMap<String, Arc<dyn WidgetProvider>>>,
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, id: &str, provider: Arc<dyn WidgetProvider>) {
        let mut providers = self.providers.write().unwrap();
        providers.insert(id.to_string(), provider);
    }

    pub fn get_all_widgets_json(&self) -> String {
        let providers = self.providers.read().unwrap();
        let mut json_widgets = Vec::new();
        
        for provider in providers.values() {
            json_widgets.push(provider.get_widget_json());
        }

        format!("[{}]", json_widgets.join(","))
    }
}
