use std::sync::{Arc, Mutex};
use crate::commands::{DashboardStatePayload, ComfortStatePayload, AmbientStatePayload, ScreenStatePayload, BrightnessStatePayload, PerformanceStatePayload, EngineHealthPayload};
use crate::intelligence::manager::{IntelligenceManager, IntelligenceContext};
use crate::intelligence::models::HistorySummary;
use tauri::Manager;

pub struct ServiceRegistry {
    pub dashboard_state: Arc<Mutex<DashboardStatePayload>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        let state = DashboardStatePayload {
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
        };

        Self {
            dashboard_state: Arc::new(Mutex::new(state)),
        }
    }

    pub fn start_hardware_worker(&self) {
        let state_clone = self.dashboard_state.clone();
        std::thread::spawn(move || {
            loop {
                // TODO: Actual Hardware polling goes here
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }
}
