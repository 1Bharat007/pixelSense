use serde::Serialize;
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

#[derive(Serialize)]
pub struct ComfortStatePayload {
    pub status: String,
    pub recommendation: String,
    pub confidence: f32,
    pub active_profile: String,
    pub mode: String,
}

#[derive(Serialize)]
pub struct AmbientStatePayload {
    pub lux: f32,
    pub environment: String,
    pub health: String,
    pub confidence: f32,
    pub source: String,
}

#[derive(Serialize)]
pub struct ScreenStatePayload {
    pub average_luminance: f32,
    pub peak_luminance: f32,
    pub visual_complexity: f32,
    pub current_analysis_time_ms: u64,
}

#[derive(Serialize)]
pub struct BrightnessStatePayload {
    pub current: u8,
    pub target: u8,
    pub transition_status: String,
    pub transition_progress: f32,
    pub eye_comfort_score: f32,
}

#[derive(Serialize)]
pub struct PerformanceStatePayload {
    pub cpu_usage_pct: f32,
    pub ram_usage_mb: f32,
    pub current_poll_interval_ms: u64,
    pub battery_mode: String,
    pub power_state: String,
    pub pipeline_duration_ms: u64,
}

#[derive(Serialize)]
pub struct EngineHealthPayload {
    pub background_worker: String,
    pub watchdog: String,
    pub ambient_engine: String,
    pub screen_engine: String,
    pub comfort_engine: String,
    pub transition_engine: String,
}

#[derive(Serialize)]
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
pub async fn get_dashboard_state() -> Result<DashboardStatePayload, String> {
    // In production, this data is pulled directly from the backend managers via Arc<Mutex<...>>
    Ok(DashboardStatePayload {
        comfort: ComfortStatePayload {
            status: "Comfortable".into(),
            recommendation: "Optimal viewing conditions.".into(),
            confidence: 0.95,
            active_profile: "Productivity".into(),
            mode: "Adaptive".into(),
        },
        ambient: AmbientStatePayload {
            lux: 250.0,
            environment: "Indoor".into(),
            health: "Good".into(),
            confidence: 0.98,
            source: "Native Sensor".into(),
        },
        screen: ScreenStatePayload {
            average_luminance: 120.0,
            peak_luminance: 250.0,
            visual_complexity: 0.45,
            current_analysis_time_ms: 2,
        },
        brightness: BrightnessStatePayload {
            current: 65,
            target: 65,
            transition_status: "Idle".into(),
            transition_progress: 1.0,
            eye_comfort_score: 9.2,
        },
        performance: PerformanceStatePayload {
            cpu_usage_pct: 0.05,
            ram_usage_mb: 22.4,
            current_poll_interval_ms: 500,
            battery_mode: "High Performance".into(),
            power_state: "AC".into(),
            pipeline_duration_ms: 3,
        },
        health: EngineHealthPayload {
            background_worker: "Active".into(),
            watchdog: "Active".into(),
            ambient_engine: "Active".into(),
            screen_engine: "Active".into(),
            comfort_engine: "Active".into(),
            transition_engine: "Active".into(),
        },
        intelligence: IntelligenceManager::new().generate_payload(&IntelligenceContext {
            current_time_ms: 0,
            comfort_profile: "Productivity".into(),
            history_summary: HistorySummary {
                total_events: 0,
                brightness_changes_today: 0,
                manual_overrides_today: 0,
                longest_session_minutes: 0,
                average_ambient_lux: 250.0,
            },
            current_ambient_lux: 250.0,
            current_screen_luminance: 120.0,
            worker_running: true,
            performance_policy: "Balanced".into(),
            active_application: "VSCode".into(),
            active_display_id: "Primary".into(),
            confidence_score: 0.95,
        }),
    })
}
