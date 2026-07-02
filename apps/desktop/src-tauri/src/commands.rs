use serde::Serialize;
use std::sync::Arc;
use crate::config::{AppConfig, ConfigService};

#[tauri::command]
pub fn get_config(config_service: tauri::State<'_, Arc<ConfigService>>) -> AppConfig {
    config_service.get_config()
}

#[tauri::command]
pub fn save_config(
    new_config: AppConfig,
    config_service: tauri::State<'_, Arc<ConfigService>>,
) -> Result<(), String> {
    config_service.save_config(new_config)
}

#[tauri::command]
pub fn preview_brightness(
    _display_id: String,
    _target: u8,
) -> Result<(), String> {
    // In a full implementation, this would delegate to AdaptiveBrightnessService.
    // For UI development, we just return Ok.
    Ok(())
}

#[tauri::command]
pub fn lock_current_comfort(
    _display_id: String,
    _profile_name: String,
) -> Result<(), String> {
    // In a full implementation, this would delegate to ComfortManager.
    // We would pass the current ambient light and screen luminance.
    // For UI development and following constraints, we just return Ok.
    Ok(())
}

#[derive(Serialize)]
pub struct ComfortStatePayload {
    pub status: String,
    pub recommendation: String,
    pub confidence: f32,
}

#[derive(Serialize)]
pub struct EngineHealthPayload {
    pub native_sensor_active: bool,
    pub screen_engine_active: bool,
    pub performance_mode: String,
}

#[tauri::command]
pub async fn get_comfort_state() -> Result<ComfortStatePayload, String> {
    Ok(ComfortStatePayload {
        status: "Comfortable".into(),
        recommendation: "Optimal viewing conditions.".into(),
        confidence: 0.95,
    })
}

#[tauri::command]
pub async fn get_engine_health() -> Result<EngineHealthPayload, String> {
    Ok(EngineHealthPayload {
        native_sensor_active: true,
        screen_engine_active: true,
        performance_mode: "AC".into(),
    })
}
