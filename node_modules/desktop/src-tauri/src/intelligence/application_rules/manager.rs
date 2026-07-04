use crate::intelligence::application_rules::models::{ApplicationCategory, ApplicationContext};

pub struct ApplicationRuleEngine;

impl ApplicationRuleEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn categorize(&self, process_name: &str) -> ApplicationCategory {
        match process_name.to_lowercase().as_str() {
            "code" | "idea64" | "studio64" => ApplicationCategory::IDE,
            "chrome" | "firefox" | "msedge" => ApplicationCategory::Browser,
            "netflix" | "vlc" => ApplicationCategory::Movie,
            "steam" | "epicgameslauncher" | "cyberpunk2077" => ApplicationCategory::Game,
            "photoshop" | "illustrator" | "premiere" => ApplicationCategory::Creative,
            "acrobat" | "kindle" => ApplicationCategory::Reading,
            _ => ApplicationCategory::Unknown,
        }
    }

    pub fn build_context(&self, process_name: &str, window_title: &str) -> ApplicationContext {
        ApplicationContext {
            process_name: process_name.to_string(),
            window_title: window_title.to_string(),
            category: self.categorize(process_name),
        }
    }
}
