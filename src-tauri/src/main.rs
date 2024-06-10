// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod system_commands; // Импортируйте модуль, содержащий функции
mod requests; // Импортируйте модуль, содержащий функции
mod config;
use config::init_config;
use dotenv::dotenv;
use log;

fn main() {
    // Вывод информации в консоль браузера
    log::info!("This is an info message");
    log::warn!("This is a warning message");
    log::error!("This is an error message");

    // Загружаем переменные из файла .env
    dotenv().ok();

    let config: config::Config = init_config();
    println!("Базовый урл {}", &config.base_url);

    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                system_commands::greet,
                system_commands::get_os_info,
                system_commands::get_current_time,
                requests::get_html_table_list_animals,
                requests::get_breeds()
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
