use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tauri::Manager;
use crate::configuration::models::AppConfig;
use crate::intelligence::manager::IntelligencePayload;
pub fn load_config_from_disk(app: &tauri::AppHandle) -> AppConfig {
    if let Ok(config_dir) = app.path().app_config_dir() {
        let config_path = config_dir.join("config.json");
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
    }
    AppConfig::default()
}

#[tauri::command]
pub fn get_config(state: tauri::State<'_, crate::registry::ServiceRegistry>) -> AppConfig {
    state.config.read().unwrap().clone()
}

#[tauri::command]
pub fn save_config(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::registry::ServiceRegistry>,
    config: serde_json::Value,
) -> Result<(), String> {
    let mut current_config = state.config.read().unwrap().clone();
    let mut current_value = serde_json::to_value(&current_config).unwrap();
    
    // Merge the partial config object into the current config
    if let (Some(current_obj), Some(new_obj)) = (current_value.as_object_mut(), config.as_object()) {
        for (k, v) in new_obj {
            current_obj.insert(k.clone(), v.clone());
        }
    }
    
    if let Ok(merged) = serde_json::from_value::<AppConfig>(current_value) {
        current_config = merged;
        
        // Update in-memory lock
        if let Ok(mut cfg) = state.config.write() {
            *cfg = current_config.clone();
        }
    }
    
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let config_path = config_dir.join("config.json");
    let content = serde_json::to_string_pretty(&current_config).map_err(|e| e.to_string())?;
    std::fs::write(config_path, content).map_err(|e| e.to_string())?;
    
    Ok(())
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
    pub confidence: Option<f32>,
    pub active_profile: String,
    pub mode: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AmbientStatePayload {
    pub lux: Option<f32>,
    pub environment: String,
    pub health: String,
    pub confidence: Option<f32>,
    pub source: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScreenStatePayload {
    pub average_luminance: Option<f32>,
    pub peak_luminance: Option<f32>,
    pub visual_complexity: Option<f32>,
    pub current_analysis_time_ms: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BrightnessStatePayload {
    pub current: Option<u8>,
    pub target: Option<u8>,
    pub transition_status: String,
    pub transition_progress: Option<f32>,
    pub eye_comfort_score: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PerformanceStatePayload {
    pub cpu_usage_pct: Option<f32>,
    pub ram_usage_mb: Option<f32>,
    pub current_poll_interval_ms: Option<u64>,
    pub battery_mode: String,
    pub power_state: String,
    pub pipeline_duration_ms: Option<u64>,
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

#[derive(Serialize, Deserialize)]
pub struct CapabilityReport {
    pub wmi_available: bool,
    pub ddc_available: bool,
    pub sensor_available: bool,
    pub internal_display: bool,
    pub supported: bool,
}

#[tauri::command]
pub async fn start_engine(state: tauri::State<'_, crate::registry::ServiceRegistry>) -> Result<CapabilityReport, String> {
    use crate::brightness::providers::windows::WindowsBrightnessProvider;
    use crate::brightness::providers::BrightnessProvider;
    use crate::display::domain::{DisplayInfo, DisplayCapabilities};
    
    let provider = WindowsBrightnessProvider::new();
    let display = DisplayInfo {
        id: "primary".to_string(),
        name: "Primary".to_string(),
        manufacturer: None,
        model: None,
        width: 1920,
        height: 1080,
        refresh_rate: None,
        is_primary: true,
        capabilities: DisplayCapabilities::default(),
    };
    
    // Check if we can read brightness
    let supported = provider.get_brightness(&display).is_ok();
    
    let wmi_available = supported; // For now WMI/DDC both return true if supported
    let ddc_available = supported; 
    let sensor_available = true; 
    let internal_display = true;
    
    if supported {
        // Start hardware engine workers asynchronously so UI never blocks
        state.start_watchdog();
        state.start_hardware_worker();
        
        if let Ok(mut lock) = state.dashboard_state.lock() {
            lock.health.ambient_engine = "Running".into();
            lock.health.transition_engine = "Running".into();
            lock.comfort.status = "Protection Enabled".into();
        }
    }
    
    Ok(CapabilityReport {
        wmi_available,
        ddc_available,
        sensor_available,
        internal_display,
        supported,
    })
}

#[tauri::command]
pub async fn stop_engine(state: tauri::State<'_, crate::registry::ServiceRegistry>) -> Result<(), String> {
    state.worker_running.store(false, std::sync::atomic::Ordering::SeqCst);
    state.watchdog_running.store(false, std::sync::atomic::Ordering::SeqCst);
    
    if let Ok(mut lock) = state.dashboard_state.lock() {
        lock.health.ambient_engine = "Stopped".into();
        lock.health.transition_engine = "Stopped".into();
        lock.comfort.status = "Protection Paused".into();
    }
    
    Ok(())
}

#[tauri::command]
pub async fn set_brightness_live(level: u8) -> Result<(), String> {
    use crate::brightness::providers::windows::WindowsBrightnessProvider;
    use crate::brightness::providers::BrightnessProvider;
    use crate::display::domain::{DisplayInfo, DisplayCapabilities};
    
    let provider = WindowsBrightnessProvider::new();
    let display = DisplayInfo {
        id: "primary".to_string(),
        name: "Primary".to_string(),
        manufacturer: None,
        model: None,
        width: 1920,
        height: 1080,
        refresh_rate: None,
        is_primary: true,
        capabilities: DisplayCapabilities::default(),
    };
    let capabilities = DisplayCapabilities {
        brightness: true,
        hdr: false,
        ddc_ci: true,
    };
    
    provider.set_brightness(&display, &capabilities, level)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_brightness() -> Result<(), String> {
    use crate::brightness::providers::windows::WindowsBrightnessProvider;
    use crate::brightness::providers::BrightnessProvider;
    use crate::display::domain::{DisplayInfo, DisplayCapabilities};
    
    let provider = WindowsBrightnessProvider::new();
    let display = DisplayInfo {
        id: "primary".to_string(),
        name: "Primary".to_string(),
        manufacturer: None,
        model: None,
        width: 1920,
        height: 1080,
        refresh_rate: None,
        is_primary: true,
        capabilities: DisplayCapabilities::default(),
    };
    let capabilities = DisplayCapabilities {
        brightness: true,
        hdr: false,
        ddc_ci: true,
    };
    
    // Get current brightness
    let original = provider.get_brightness(&display).unwrap_or(50);
    
    // Set to 20
    let _ = provider.set_brightness(&display, &capabilities, 20);
    
    // Wait
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    
    // Restore
    let _ = provider.set_brightness(&display, &capabilities, original);
    
    Ok(())
}
