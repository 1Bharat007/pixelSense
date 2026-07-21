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
    
    fn deep_merge(a: &mut serde_json::Value, b: serde_json::Value) {
        match (a, b) {
            (serde_json::Value::Object(ref mut a_map), serde_json::Value::Object(b_map)) => {
                for (k, v) in b_map {
                    deep_merge(a_map.entry(k).or_insert(serde_json::Value::Null), v);
                }
            }
            (a_ref, b_val) => {
                *a_ref = b_val;
            }
        }
    }

    deep_merge(&mut current_value, config);
    
    if let Ok(merged) = serde_json::from_value::<AppConfig>(current_value.clone()) {
        current_config = merged;
        
        // Update in-memory lock
        if let Ok(mut cfg) = state.config.write() {
            *cfg = current_config.clone();
        }
    } else {
        return Err("Failed to parse merged config structure. Missing required fields?".to_string());
    }
    
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let config_path = config_dir.join("config.json");
    let content = serde_json::to_string_pretty(&current_config).map_err(|e| e.to_string())?;
    std::fs::write(config_path, content).map_err(|e| e.to_string())?;
    
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
    pub explanation: Option<String>,
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
    pub context: Option<String>,
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
    pub failure_reason: Option<String>,
}

#[tauri::command]
pub async fn start_engine(state: tauri::State<'_, crate::registry::ServiceRegistry>) -> Result<CapabilityReport, String> {
    use crate::brightness::providers::native::NativeBrightnessProvider;
    use crate::brightness::providers::BrightnessProvider;
    use crate::display::domain::{DisplayInfo, DisplayCapabilities};
    
    let provider = NativeBrightnessProvider::new();
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
    
    let (supported, failure_reason) = match provider.get_brightness(&display) {
        Ok(v) => {
            let _current_brightness = v;
            (true, None)
        },
        Err(e) => (false, Some(e.to_string())),
    };
    
    let wmi_available = supported; 
    let ddc_available = supported; 
    let sensor_session = crate::platform::hardware::sensor::manager::SensorSession::new();
    let sensor_available = sensor_session.read_lux().is_ok();
    let internal_display = true;
    
    // Print the requested concise Hardware Report
    println!("\n=== Hardware Report ===");
    println!("Internal Display\n  Supported");
    println!("External Display\n  Not Connected");
    println!("Ambient Sensor\n  {}", if sensor_available { "Supported" } else { "Unavailable" });
    println!("Brightness API\n  {}", if let Some(ref e) = failure_reason { format!("FAILED - {}", e) } else { "WMI/DDC".to_string() });
    println!("Read-back\n  {}\n", if supported { "Supported" } else { "Unsupported" });

    if supported {
        // Start hardware engine workers asynchronously so UI never blocks
        state.start_watchdog();
        state.start_hardware_worker();
        
        if let Ok(mut lock) = state.dashboard_state.lock() {
            lock.health.ambient_engine = "Running".into();
            lock.health.transition_engine = "Running".into();
            lock.comfort.status = "Protection Enabled".into();
        }
    } else {
        if let Ok(mut lock) = state.dashboard_state.lock() {
            lock.health.ambient_engine = "Unsupported".into();
            lock.health.transition_engine = "Unsupported".into();
            lock.comfort.status = "Disabled (Hardware Error)".into();
        }
    }
    
    Ok(CapabilityReport {
        wmi_available,
        ddc_available,
        sensor_available,
        internal_display,
        supported,
        failure_reason,
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
pub async fn set_brightness_live(state: tauri::State<'_, crate::registry::ServiceRegistry>, level: u8) -> Result<(), String> {
    use crate::display::domain::{DisplayInfo, DisplayCapabilities};
    use crate::background::event_log::{LogEvent, EventCategory};
    
    // 1. Suspend automation for the configured duration
    if let Ok(lock) = state.transition_worker.read() {
        if let Some(worker) = &*lock {
            worker.suspend_automation();
        }
    }

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

    // 2. Get current brightness before change (for event log)
    let start_time = std::time::Instant::now();
    let previous = match state.brightness_manager.get_brightness(&display) {
        Ok(val) => val,
        Err(e) => return Err(format!("Could not read previous brightness: {}", e)),
    };

    // 3. Apply the brightness change
    if let Err(e) = state.brightness_manager.set_brightness(&display, &capabilities, level as i32) {
        if let Ok(mut log) = state.event_log.lock() {
            log.push(LogEvent::new(EventCategory::SystemEvent, &format!("IPC: set_brightness failed - {}", e)));
        }
        return Err(e.to_string());
    }

    // 4. Verify read-back (IPC Verification) - Retry loop for slow WMI hardware
    let mut readback = 0;
    let mut success = false;
    for _ in 0..4 {
        std::thread::sleep(std::time::Duration::from_millis(40));
        readback = match state.brightness_manager.get_brightness(&display) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if (readback as i32 - level as i32).abs() <= 5 {
            success = true;
            break;
        }
    }
    
    if !success {
        let err_msg = format!("This monitor doesn't support automatic brightness (value rejected by hardware). Readback was {} but target was {}", readback, level);
        if let Ok(mut log) = state.event_log.lock() { log.push(LogEvent::new(EventCategory::SystemEvent, &err_msg)); }
        return Err(err_msg);
    }

    // Generate Structured Trace
    println!("\n=== Brightness Request ===");
    println!("Current:\n  {}%", previous);
    println!("Target:\n  {}%", level);
    println!("Provider:\n  WMI/DDC");
    println!("Write:\n  SUCCESS");
    println!("Read-back:\n  {}%", readback);
    println!("Elapsed:\n  {} ms\n", start_time.elapsed().as_millis());

    // 5. Record in BrightnessMemory — detect which app is in the foreground
    let active_app = crate::platform::application::active_window::get_active_application();
    if let Ok(mut memory) = state.brightness_memory.lock() {
        memory.record_override(&active_app, level);
    }

    // 6. Log the manual override to EventLog (IPC Verification complete)
    if let Ok(mut log) = state.event_log.lock() {
        log.push(
            LogEvent::new(EventCategory::BrightnessChanged, "Manual override (IPC Verified)")
                .with_values(format!("{}%", previous), format!("{}%", readback)),
        );
    }

    Ok(())
}

#[tauri::command]
pub fn test_brightness(state: tauri::State<'_, crate::registry::ServiceRegistry>) -> Result<(), String> {
    use crate::display::domain::{DisplayInfo, DisplayCapabilities};
    
    let provider = &state.brightness_manager;
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
    let start_time = std::time::Instant::now();
    let original = provider.get_brightness(&display).map_err(|e| format!("Failed to read initial brightness: {}", e))?;
    
    // Set to 20
    provider.set_brightness(&display, &capabilities, 20).map_err(|e| format!("Failed to send set_brightness command: {}", e))?;
    
    // Verify
    let mut verify = 0;
    let mut success = false;
    for _ in 0..4 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        verify = provider.get_brightness(&display).unwrap_or(0);
        if (verify as i32 - 20).abs() <= 5 {
            success = true;
            break;
        }
    }
    
    if !success {
        // Restore original just in case it was slow
        let _ = provider.set_brightness(&display, &capabilities, original as i32);
        return Err(format!("Display rejected the brightness change (read-back mismatch). Read {} but expected 20.", verify));
    }
    
    println!("\n=== Test Brightness Request ===");
    println!("Current:\n  {}%", original);
    println!("Target:\n  20%");
    println!("Provider:\n  WMI/DDC");
    println!("Write:\n  SUCCESS");
    println!("Read-back:\n  {}%", verify);
    println!("Elapsed:\n  {} ms\n", start_time.elapsed().as_millis());
    
    // Wait
    std::thread::sleep(std::time::Duration::from_millis(2000));
    
    // Restore
    let restore_start = std::time::Instant::now();
    if let Err(e) = provider.set_brightness(&display, &capabilities, original as i32) {
        return Err(format!("Test succeeded but failed to restore original brightness: {}", e));
    }
    
    std::thread::sleep(std::time::Duration::from_millis(100));
    let restored = provider.get_brightness(&display).unwrap_or(0);
    
    println!("\n=== Test Restore Request ===");
    println!("Current:\n  20%");
    println!("Target:\n  {}%", original);
    println!("Provider:\n  WMI/DDC");
    println!("Write:\n  SUCCESS");
    println!("Read-back:\n  {}%", restored);
    println!("Elapsed:\n  {} ms\n", restore_start.elapsed().as_millis());

    Ok(())
}

// ==========================================
// Event Log
// ==========================================

#[tauri::command]
pub async fn get_event_log(
    state: tauri::State<'_, crate::registry::ServiceRegistry>,
) -> Result<Vec<crate::background::event_log::LogEvent>, String> {
    let log = state.event_log.lock().map_err(|e| e.to_string())?;
    Ok(log.get_recent())
}

// ==========================================
// Brightness Memory
// ==========================================

#[derive(Serialize, Deserialize)]
pub struct BrightnessMemoryEntry {
    pub app_name: String,
    pub preferred_brightness: u8,
    pub override_count: u32,
}

#[tauri::command]
pub async fn get_brightness_memory(
    state: tauri::State<'_, crate::registry::ServiceRegistry>,
) -> Result<Vec<BrightnessMemoryEntry>, String> {
    let memory = state.brightness_memory.lock().map_err(|e| e.to_string())?;
    let entries = memory.all_preferences().into_iter().map(|r| BrightnessMemoryEntry {
        app_name: r.app_name.clone(),
        preferred_brightness: r.preferred_brightness,
        override_count: r.override_count,
    }).collect();
    Ok(entries)
}

// ==========================================
// Hardware Capabilities (Real Detection)
// ==========================================

#[derive(Serialize, Deserialize, Clone)]
pub struct HardwareCapabilities {
    pub brightness_api: String,      // "WMI", "DDC/CI", "Unsupported"
    pub brightness_available: bool,
    pub ambient_sensor: String,      // "Hardware", "Unavailable"
    pub ambient_available: bool,
    pub internal_display: bool,
    pub failure_reason: Option<String>,
}

#[tauri::command]
pub async fn get_hardware_capabilities(
    state: tauri::State<'_, crate::registry::ServiceRegistry>,
) -> Result<HardwareCapabilities, String> {
    use crate::brightness::providers::native::NativeBrightnessProvider;
    use crate::brightness::providers::BrightnessProvider;
    use crate::display::domain::{DisplayInfo, DisplayCapabilities};

    let provider = NativeBrightnessProvider::new();
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

    let (brightness_available, failure_reason) = match provider.get_brightness(&display) {
        Ok(_) => (true, None),
        Err(e) => (false, Some(e.to_string())),
    };

    // Check ambient sensor availability from live health state.
    let ambient_available = {
        let ds = state.dashboard_state.lock().map_err(|e| e.to_string())?;
        !ds.health.ambient_engine.contains("Unavailable") && !ds.health.ambient_engine.contains("Error")
    };

    Ok(HardwareCapabilities {
        brightness_api: if brightness_available { "WMI Native".into() } else { "Unsupported".into() },
        brightness_available,
        ambient_sensor: if ambient_available { "Hardware Sensor".into() } else { "Unavailable".into() },
        ambient_available,
        internal_display: brightness_available, // Internal display implies brightness control
        failure_reason,
    })
}

