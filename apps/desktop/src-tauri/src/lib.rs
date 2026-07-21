#![allow(clippy::new_without_default)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::single_match)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::cast_abs_to_unsigned)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::cmp_null)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::manual_checked_ops)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_borrows_for_generic_args)]

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
pub mod adaptation;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::Manager;
use registry::ServiceRegistry;
use std::time::Instant;

pub fn run(start_time: Instant) {
    println!("{}ms | Tauri Builder created", start_time.elapsed().as_millis());

    let mut builder = tauri::Builder::default();

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
    
    println!("{}ms | Plugins initialized", start_time.elapsed().as_millis());

    builder
        .setup(move |app| {
            println!("{}ms | Setup entered", start_time.elapsed().as_millis());

            // 1. Logger (already initialized via plugin)
            // 2. Config
            let initial_config = commands::load_config_from_disk(app.handle());

            // 3. Main Window
            let window = match app.get_webview_window("main") {
                Some(w) => w,
                None => {
                    eprintln!("FATAL ERROR: Main window could not be retrieved from Tauri.");
                    let _ = std::fs::write("startup.log", "FATAL: Main window missing\n");
                    
                    #[cfg(windows)]
                    {
                        let _ = std::process::Command::new("powershell")
                            .args([
                                "-Command",
                                "Add-Type -AssemblyName PresentationFramework; [System.Windows.MessageBox]::Show('The main window failed to initialize. PixelSense cannot continue.', 'PixelSense Startup Error', 'OK', 'Error')"
                            ])
                            .spawn();
                    }
                    std::process::exit(1);
                }
            };
            
            // 4. Show Window
            if let Err(e) = window.show() {
                eprintln!("FATAL ERROR: Failed to show window: {e}");
                let _ = std::fs::write("startup.log", format!("FATAL: window.show() failed: {e}\n"));
                std::process::exit(1);
            }
            println!("{}ms | Window Visible", start_time.elapsed().as_millis());

            // 5. Focus Window
            if let Err(e) = window.set_focus() {
                println!("Warning: window.set_focus() failed: {e}");
            }

            // 6. Render React
            println!("{}ms | React Loaded (Backend Ready)", start_time.elapsed().as_millis());

            // 7. Inject ServiceRegistry
            let is_adaptive_enabled = initial_config.adaptive.enabled;
            let registry = ServiceRegistry::new(initial_config);
            app.manage(registry);

            let state = app.state::<ServiceRegistry>();
            
            // 8. Auto-start Engine if Previously Enabled
            if is_adaptive_enabled {
                println!("{}ms | Engine Started", start_time.elapsed().as_millis());
                let state_clone = state.inner();
                state_clone.start_watchdog();
                state_clone.start_hardware_worker();
                println!("{}ms | Hardware Ready", start_time.elapsed().as_millis());
            }

            // 9. Automated Soak Test Snapshots
            let state_for_soak = state.inner().clone();
            std::thread::spawn(move || {
                let mut sys = sysinfo::System::new_all();
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(600));
                    sys.refresh_all();
                    
                    let pid = sysinfo::get_current_pid().unwrap();
                    let process = sys.process(pid);
                    let cpu = process.map(|p| p.cpu_usage()).unwrap_or(0.0);
                    let ram = process.map(|p| p.memory()).unwrap_or(0) / 1024 / 1024;
                    
                    let health = if let Ok(ds) = state_for_soak.dashboard_state.lock() {
                        format!("Ambient: {} | Transition: {}", ds.health.ambient_engine, ds.health.transition_engine)
                    } else {
                        "Locked".into()
                    };
                    
                    println!("\n=== SOAK TEST 10-MIN SNAPSHOT ===");
                    println!("RAM: {} MB", ram);
                    println!("CPU: {:.1}%", cpu);
                    println!("Engine state: {}", health);
                    
                    if let Ok(log) = state_for_soak.event_log.lock() {
                        if let Some(last_error) = log.get_recent().iter().find(|e| e.description.to_lowercase().contains("error") || e.description.to_lowercase().contains("fail")) {
                            println!("Last hardware error: {}", last_error.description);
                        } else {
                            println!("Last hardware error: None");
                        }
                    }
                    println!("=================================\n");
                }
            });

            // 10. Initialize Tray
            if let Err(e) = tray::create_tray(app.handle()) {
                println!("Warning: Failed to initialize tray: {e}");
            }

            println!("{}ms | Startup completed", start_time.elapsed().as_millis());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::lock_current_comfort,
            commands::get_dashboard_state,
            commands::get_history,
            commands::get_notifications,
            commands::start_engine,
            commands::stop_engine,
            commands::set_brightness_live,
            commands::test_brightness,
            commands::get_event_log,
            commands::get_brightness_memory,
            commands::get_hardware_capabilities
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










