// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod system_commands; // Импортируйте модуль, содержащий функции

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                system_commands::greet,
                system_commands::get_os_info,
                system_commands::get_current_time
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
