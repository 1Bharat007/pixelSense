use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use crate::configuration::models::AppConfig;
use crate::intelligence::manager::{IntelligenceManager, IntelligencePayload};
use crate::intelligence::models::{IntelligenceContext, HistorySummary};
#[tauri::command]
pub fn get_config() -> AppConfig {
    AppConfig::default() // Mocked pending full ServiceRegistry integration
}

#[tauri::command]
pub fn save_config(
    _new_config: AppConfig,
) -> Result<(), String> {
    Ok(()) // Mocked pending full ServiceRegistry integration
}

#[tauri::command]
pub fn preview_brightness(
    _display_id: String,
    _target: u8,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn lock_current_comfort(
    _display_id: String,
    _profile_name: String,
) -> Result<(), String> {
    Ok(())
}

// ==========================================
// IPC Models for Dashboard
// ==========================================

#[derive(Serialize, Deserialize, Clone)]
pub struct ComfortStatePayload {
    pub status: String,
    pub recommendation: String,
    pub confidence: f32,
    pub active_profile: String,
    pub mode: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AmbientStatePayload {
    pub lux: f32,
    pub environment: String,
    pub health: String,
    pub confidence: f32,
    pub source: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScreenStatePayload {
    pub average_luminance: f32,
    pub peak_luminance: f32,
    pub visual_complexity: f32,
    pub current_analysis_time_ms: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BrightnessStatePayload {
    pub current: u8,
    pub target: u8,
    pub transition_status: String,
    pub transition_progress: f32,
    pub eye_comfort_score: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PerformanceStatePayload {
    pub cpu_usage_pct: f32,
    pub ram_usage_mb: f32,
    pub current_poll_interval_ms: u64,
    pub battery_mode: String,
    pub power_state: String,
    pub pipeline_duration_ms: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EngineHealthPayload {
    pub background_worker: String,
    pub watchdog: String,
    pub ambient_engine: String,
    pub screen_engine: String,
    pub comfort_engine: String,
    pub transition_engine: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DashboardStatePayload {
    pub comfort: ComfortStatePayload,
    pub ambient: AmbientStatePayload,
    pub screen: ScreenStatePayload,
    pub brightness: BrightnessStatePayload,
    pub performance: PerformanceStatePayload,
    pub health: EngineHealthPayload,
    pub intelligence: IntelligencePayload,
}

#[tauri::command]
pub async fn get_dashboard_state(state: tauri::State<'_, crate::registry::ServiceRegistry>) -> Result<DashboardStatePayload, String> {
    let dashboard_state = state.dashboard_state.lock().unwrap();
    Ok(dashboard_state.clone())
}

// ==========================================
// IPC Models for History & Notifications
// ==========================================

#[derive(Serialize, Deserialize)]
pub struct HistoryEvent {
    pub id: String,
    pub timestamp: u64,
    pub category: String,
    pub description: String,
    pub before_value: Option<String>,
    pub after_value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NotificationEvent {
    pub id: String,
    pub timestamp: u64,
    pub priority: String,
    pub title: String,
    pub message: String,
    pub read: bool,
    pub action_type: Option<String>,
}

#[tauri::command]
pub fn get_history() -> Result<Vec<HistoryEvent>, String> {
    let path = PathBuf::from("history.jsonl");
    let mut events = Vec::new();
    
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_str) = line {
                if let Ok(event) = serde_json::from_str::<HistoryEvent>(&line_str) {
                    events.push(event);
                }
            }
        }
    }
    
    // Sort descending by timestamp
    events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(events)
}

#[tauri::command]
pub fn get_notifications() -> Result<Vec<NotificationEvent>, String> {
    let path = PathBuf::from("notifications.jsonl");
    let mut events = Vec::new();
    
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_str) = line {
                if let Ok(event) = serde_json::from_str::<NotificationEvent>(&line_str) {
                    events.push(event);
                }
            }
        }
    }
    
    events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(events)
}
