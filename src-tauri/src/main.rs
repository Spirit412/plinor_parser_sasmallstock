// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod controllers; // Импортируйте модуль, содержащий функции
mod config;
pub mod logging;

use logging::{ init_logging, log_with_context };
use dotenv::dotenv;

use tauri::{ Manager, Runtime };

#[tauri::command]
async fn long_running_job<R: Runtime>(window: tauri::Window<R>) {
    for i in 0..101 {
        window.emit("progress", i).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}

fn main() {
    init_logging();
    log_with_context(module_path!(), line!(), "Starting!!");
    // Загружаем переменные из файла .env
    dotenv().ok();
    let base_url = &config::CONFIG.base_url;
    println!("Базовый урл {}", base_url);

    tauri::Builder
        ::default()
        .setup(|app| {
            app.listen_global("age", |event| {
                println!("age: {}", event.payload().unwrap());
            });
            Ok(())
        })
        .invoke_handler(
            tauri::generate_handler![
                controllers::get_breeds,
                controllers::get_os_info,
                controllers::get_current_time,
                long_running_job
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
