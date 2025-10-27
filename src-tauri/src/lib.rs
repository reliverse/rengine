mod types;
mod commands;

use commands::AppState;
use tauri::async_runtime::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::create_new_map,
            commands::load_map,
            commands::save_map,
            commands::add_object,
            commands::update_object,
            commands::delete_object,
            commands::get_current_map,
            commands::set_current_map,
            commands::sync_map_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
