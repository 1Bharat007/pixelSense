use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApplicationCategory {
    IDE,
    Browser,
    Movie,
    Game,
    Creative,
    Reading,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationContext {
    pub process_name: String,
    pub window_title: String,
    pub category: ApplicationCategory,
}
