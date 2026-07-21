// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;

fn main() {
    // 1. Global Panic Handler (Rule #8)
    std::panic::set_hook(Box::new(|info| {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic"
        };
        
        let location = info.location().map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column())).unwrap_or_else(|| "unknown".into());
        let msg = format!("[{}] PANIC at {}: {}\n", timestamp, location, payload);
        
        let _ = std::fs::write("panic.log", msg.clone());
        eprintln!("{}", msg);
    }));

    // 2. Startup Metrics (Rule #6)
    println!("=== STARTUP TIMING ===");
    let start_time = Instant::now();
    println!("{}ms | Binary started", start_time.elapsed().as_millis());

    app_lib::run(start_time);
}
