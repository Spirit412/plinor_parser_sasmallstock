// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod controllers; // Импортируйте модуль, содержащий функции
mod config;
pub mod logging;
use tauri::AppHandle;
use logging::{init_logging, log_with_context};
use dotenv::dotenv;

use tauri::{Manager, Runtime};
use serde::{Deserialize, Serialize};
// use uuid::Uuid;
use tokio::sync::Mutex;
use std::sync::Arc;

// Структура для сериализации и десериализации сообщений
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    function: String, // Название функции
    data: serde_json::Value, // Данные в формате JSON
    uuid4: String, // Идентификатор сообщения
    response_queue: String, // Имя очереди для ответа
}


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
    tauri::Builder::default()
        .setup(|app| {
            // Создаем Arc<Mutex<AppHandle>> для безопасного доступа к AppHandle из асинхронной задачи
            let app_handle = Arc::new(Mutex::new(app.handle()));
            let app_handle_clone = app_handle.clone();
            // Очередь для сообщений от фронтенда к бекенду
            app.listen_global("frontend_to_backend", move |event| {
                let app_handle = app_handle_clone.clone();
                tauri::async_runtime::spawn(async move {
                    if let Some(payload) = event.payload() {
                        // Десериализация сообщения
                        let message: Message = serde_json::from_str(payload).unwrap();
                        // Обработка сообщения
                        handle_frontend_message(app_handle.clone(), message).await;
                    }
                });
            });
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
                controllers::set_animals_to_db,
                long_running_job
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Асинхронная функция для обработки сообщений от фронтенда
async fn handle_frontend_message(app_handle: Arc<Mutex<AppHandle>>, message: Message) {
    match message.function.as_str() {
        "click_button" => {
            let result = click_button(message.data).await;
            let response_queue = message.response_queue.clone(); // Clone the response_queue
            let response = Message {
                function: "front_click_button".to_string(),
                data: result,
                uuid4: message.uuid4,
                response_queue: response_queue.clone(), // Use the cloned response_queue
            };
            let app = app_handle.lock().await;
            app.emit_all(&response_queue, serde_json::to_string(&response).unwrap()).unwrap(); // Use the cloned response_queue
        }
        _ => {
            // Обработка других функций
        }
    }
}

// Асинхронная функция для обработки данных
async fn click_button(data: serde_json::Value) -> serde_json::Value {
    // Обработка данных и возвращение результата в формате JSON
    serde_json::json!({
        "processed_data": format!("{}-от бекенда", data["text"].as_str().unwrap_or_default())
    })
}