pub mod brightness;
pub mod display;
pub mod commands;
pub mod ambient;
pub mod visual_comfort;
pub mod configuration;
pub mod platform;
pub mod transition;
pub mod decision;
pub mod adaptive;
pub mod screen_analysis;
pub mod background;
pub mod performance;
pub mod tray;
pub mod experience;
pub mod intelligence;
pub mod core;
pub mod plugin;
pub mod governance;
pub mod security;
pub mod crash;
pub mod installer;
pub mod update;
pub mod logging;
pub mod diagnostics;
pub mod dashboard;
pub mod registry;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::Manager;
use registry::ServiceRegistry;

pub fn run() {
  tauri::Builder::default()
        .setup(|app| {
            let registry = ServiceRegistry::new();
            registry.start_hardware_worker();
            app.manage(registry);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::preview_brightness,
            commands::lock_current_comfort,
            commands::get_dashboard_state,
            commands::get_history,
            commands::get_notifications
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










