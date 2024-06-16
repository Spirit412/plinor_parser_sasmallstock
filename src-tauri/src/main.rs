// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod controllers; // Импортируйте модуль, содержащий функции
mod config;
pub mod logging;

use logging::{ init_logging, log_with_context };
use dotenv::dotenv;

fn main() {
    init_logging();
    log_with_context(module_path!(), line!(), "Starting!!");
    // Загружаем переменные из файла .env
    dotenv().ok();
    let base_url = &config::CONFIG.base_url;
    println!("Базовый урл {}", base_url);

    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                controllers::get_breeds,
                controllers::get_os_info,
                controllers::get_current_time
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
