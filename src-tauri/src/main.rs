// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::SystemTray;

mod tray;
mod version;

fn main() {
    tauri::Builder::default()
        .system_tray(SystemTray::new())
        .setup(|app| {
            tray::setup(app)?;
            Ok(())
        })
        .on_system_tray_event(tray::on_menu_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
