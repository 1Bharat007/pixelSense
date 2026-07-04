use serde::{Deserialize, Serialize};

/// Internal Platform representation of a display.
/// This model must not be exposed outside the Platform layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NativeDisplay {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub position_x: i32,
    pub position_y: i32,
    pub refresh_rate: Option<f32>,
    pub is_primary: bool,
}
