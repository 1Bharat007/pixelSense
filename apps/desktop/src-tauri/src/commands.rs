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
