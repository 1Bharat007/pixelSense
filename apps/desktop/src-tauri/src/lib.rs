pub mod brightness;
pub mod capabilities;
pub mod display;
pub mod commands;
pub mod ambient;
pub mod comfort;
pub mod visual_comfort;
pub mod config;
pub mod platform;
pub mod transition;
pub mod decision;
pub mod adaptive;
pub mod screen_analysis;
pub mod background;
pub mod performance;
pub mod tray;
pub mod experience;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::Manager;

pub fn run() {
  tauri::Builder::default()
        .setup(|app| {
            let config_path = app.path().app_config_dir().unwrap_or_default().join("config.json");
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent).unwrap_or_default();
            }
            let config_service = std::sync::Arc::new(config::ConfigService::new(config_path));
            app.manage(config_service);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::preview_brightness,
            commands::lock_current_comfort,
            commands::get_dashboard_state
        ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      // Initialize System Tray
      tray::create_tray(app.handle()).expect("Failed to initialize tray");
      
      Ok(())
    })
    .on_window_event(|window, event| match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}










