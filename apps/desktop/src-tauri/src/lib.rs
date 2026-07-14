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
    println!("[1] Binary started");

    let mut builder = tauri::Builder::default();
    println!("[2] Builder created");

    // Initialize logger first
    if cfg!(debug_assertions) {
        builder = builder.plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build()
        );
    }
    
    // Initialize single-instance plugin
    builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
        println!("Second instance launched. Restoring existing window...");
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.unminimize();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }));
    
    println!("[3] Plugins initialized");

    builder
        .setup(|app| {
            println!("[4] Setup entered");

            // 1. Logger (already initialized via plugin)
            // 2. Config
            let initial_config = commands::load_config_from_disk(app.handle());

            // 3. Main Window
            let window = match app.get_webview_window("main") {
                Some(w) => w,
                None => {
                    eprintln!("FATAL ERROR: Main window could not be retrieved from Tauri.");
                    std::fs::write("startup.log", "FATAL: Main window missing\n").unwrap_or_default();
                    
                    #[cfg(windows)]
                    {
                        std::process::Command::new("powershell")
                            .args(&[
                                "-Command",
                                "Add-Type -AssemblyName PresentationFramework; [System.Windows.MessageBox]::Show('The main window failed to initialize. PixelSense cannot continue.', 'PixelSense Startup Error', 'OK', 'Error')"
                            ])
                            .spawn()
                            .ok();
                    }
                    std::process::exit(1);
                }
            };
            println!("[5] Main window retrieved");
            
            let is_visible = window.is_visible().unwrap_or(false);
            println!("[6] Window state before show: visible={}", is_visible);

            // 4. Show Window
            if let Err(e) = window.show() {
                eprintln!("FATAL ERROR: Failed to show window: {e}");
                std::fs::write("startup.log", format!("FATAL: window.show() failed: {e}\n")).unwrap_or_default();
                std::process::exit(1);
            }
            println!("[7] window.show() called");

            // 5. Focus Window
            if let Err(e) = window.set_focus() {
                // Focus refusal is common on Windows (e.g. if another app steals focus), we don't strictly exit, but we log it.
                println!("Warning: window.set_focus() failed: {e}");
            }
            println!("[8] window.set_focus() called");

            // 6. Render React
            println!("[9] React frontend loaded");

            // 7. Inject ServiceRegistry
            let is_adaptive_enabled = initial_config.adaptive.enabled;
            let registry = ServiceRegistry::new(initial_config);
            app.manage(registry);

            let state = app.state::<ServiceRegistry>();
            
            // 8. Auto-start Engine if Previously Enabled
            if is_adaptive_enabled {
                println!("[10] Engine auto-start requested from config");
                let state_clone = state.inner().clone();
                tauri::async_runtime::spawn(async move {
                    // Start in background without blocking UI
                    state_clone.start_watchdog();
                    state_clone.start_hardware_worker();
                });
            } else {
                println!("[10] Engine remains dormant (disabled in config)");
            }

            // 10. Initialize Tray
            if let Err(e) = tray::create_tray(app.handle()) {
                println!("Warning: Failed to initialize tray: {e}");
            }
            println!("[11] Tray initialized");

            println!("[12] Startup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::preview_brightness,
            commands::lock_current_comfort,
            commands::get_dashboard_state,
            commands::get_history,
            commands::get_notifications,
            commands::start_engine,
            commands::stop_engine,
            commands::set_brightness_live,
            commands::test_brightness
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let _ = window.hide();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}










