use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let title_i = MenuItem::with_id(app, "title", "PixelSense", false, None::<&str>)?;
    let status_i = MenuItem::with_id(app, "status", "Protection Active", false, None::<&str>)?;
    let room_i = MenuItem::with_id(app, "room", "Room Lighting: Watching", false, None::<&str>)?;
    let pause_i = MenuItem::with_id(app, "pause", "Pause Protection", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Open Dashboard", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
    
    let menu = Menu::with_items(app, &[&title_i, &status_i, &room_i, &pause_i, &show_i, &quit_i])?;
    
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "pause" => {
                // Future: Add IPC channel to pause the background worker
                // For now, opening the dashboard allows pausing via UI.
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;
        
    Ok(())
}
