use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let title_i = MenuItem::with_id(app, "title", "PixelSense", false, None::<&str>)?;
    let comfort_i = MenuItem::with_id(app, "comfort", "Comfort: --", false, None::<&str>)?;
    let brightness_i = MenuItem::with_id(app, "brightness", "Brightness: --", false, None::<&str>)?;
    let context_i = MenuItem::with_id(app, "context", "Context: --", false, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Open Dashboard", true, None::<&str>)?;
    let pause_i = MenuItem::with_id(app, "pause", "Pause", true, None::<&str>)?;
    let resume_i = MenuItem::with_id(app, "resume", "Resume", true, None::<&str>)?;
    let manual_i = MenuItem::with_id(app, "manual", "Manual Mode", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
    
    let menu = Menu::with_items(app, &[
        &title_i, 
        &comfort_i, 
        &brightness_i, 
        &context_i, 
        &show_i, 
        &pause_i, 
        &resume_i, 
        &manual_i, 
        &quit_i
    ])?;
    
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" | "pause" | "resume" | "manual" => {
                // For now, opening the dashboard allows pausing/resuming via UI.
                if let Some(window) = app.get_webview_window("main") {
                    if let Err(e) = window.show() {
                        log::warn!("Failed to show window from tray: {}", e);
                    }
                    if let Err(e) = window.set_focus() {
                        log::warn!("Failed to focus window from tray: {}", e);
                    }
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
                    if let Err(e) = window.show() {
                        log::warn!("Failed to show window from tray: {}", e);
                    }
                    if let Err(e) = window.set_focus() {
                        log::warn!("Failed to focus window from tray: {}", e);
                    }
                }
            }
        })
        .build(app)?;
        
    Ok(())
}
