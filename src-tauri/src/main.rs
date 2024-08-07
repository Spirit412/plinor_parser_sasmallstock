// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_variables, unused_imports)]
mod controllers; // Импортируйте модуль, содержащий функции
mod config;
mod logging;
mod database;

use tauri::AppHandle;
use dotenv::dotenv;
use tracing::{ info, error };
use tauri::{ Manager, Runtime };
use serde::{ Deserialize, Serialize };
// use uuid::Uuid;
use tokio::sync::Mutex;
use std::sync::Arc;

/// Структура сообщения
///
/// Представляет сообщение, которое может быть использовано для передачи данных между компонентами приложения.
///
/// # Поля структуры
///
/// - `function`: строка, название функции, которая должна быть вызвана
/// - `data`: `serde_json::Value`, данные в формате JSON, которые будут переданы в функцию
/// - `uuid4`: строка, уникальный идентификатор сообщения
/// - `response_queue`: строка, имя очереди, в которую будет отправлен ответ
///
/// # Примеры
///
/// ```
/// use serde_json::json;
///
/// let message = Message {
///     function: "my_function".to_string(),
///     data: json!({"key": "value"}),
///     uuid4: "123e4567-e89b-12d3-a456-426655440000".to_string(),
///     response_queue: "my_queue".to_string(),
/// };
///
/// println!("{:?}", message);
/// ```
// Структура для сериализации и десериализации сообщений
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    function: String, // Название функции
    data: serde_json::Value, // Данные в формате JSON
    uuid4: String, // Идентификатор сообщения
    response_queue: String, // Имя очереди для ответа
}

/// Имитирует длительный процесс
///
/// Имитирует длительный процесс, отправляя прогресс в окно через событие "progress".
///
/// # Параметры
///
/// - `window`: `tauri::Window<R>`, окно, в котором будет отправляться прогресс
///
/// # Примеры
///
/// ```
/// use tauri::Runtime;
///
/// #[tauri::command]
/// async fn long_running_job<R: Runtime>(window: tauri::Window<R>) {
///     for i in 0..101 {
///         window.emit("progress", i).unwrap();
///         std::thread::sleep(std::time::Duration::from_millis(40));
///     }
/// }
/// ```
#[tauri::command]
async fn long_running_job<R: Runtime>(window: tauri::Window<R>) {
    for i in 0..101 {
        window.emit("progress", i).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}

fn main() {
    dotenv::from_path(".env").ok();
    dotenv().ok();

    logging::init_logging();

    // Использование tracing
    info!("Application started");

    tauri::Builder
        ::default()
        .setup(|app| {
            // Initialize the database.
            database::db::init_db();

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

/// Асинхронная функция для обработки сообщений от фронтенда
///
/// Обрабатывает сообщение от фронтенда и выполняет соответствующую функцию.
///
/// # Параметры
///
/// - `app_handle`: `Arc<Mutex<AppHandle>>`, управляющая приложением
/// - `message`: `Message`, сообщение от фронтенда
///
/// # Примеры
///
/// ```
/// use serde_json::to_string;
///
/// async fn handle_frontend_message(app_handle: Arc<Mutex<AppHandle>>, message: Message) {
///     match message.function.as_str() {
///         "click_button" => {
///             let result = click_button(message.data).await;
///             let response_queue = message.response_queue.clone();
///             let response = Message {
///                 function: "front_click_button".to_string(),
///                 data: result,
///                 uuid4: message.uuid4,
///                 response_queue: response_queue.clone(),
///             };
///             let app = app_handle.lock().await;
///             app.emit_all(&response_queue, to_string(&response).unwrap()).unwrap();
///         }
///         _ => {
///             // Обработка других функций
///         }
///     }
/// }
/// ```
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

/// Асинхронная функция для обработки данных и возврата результата в формате JSON.
///
/// # Аргументы
///
/// * `data` - Данные для обработки в формате serde_json::Value.
///
/// # Возвращает
///
/// Возвращает обработанные данные в формате JSON с определенной структурой.
///
/// # Примеры
///
/// ```
/// let data = serde_json::json!({"text": "пример"});
/// let result = click_button(data).await;
/// assert_eq!(result["processed_data"], "пример-от бекенда");
///
async fn click_button(data: serde_json::Value) -> serde_json::Value {
    // Обработка данных и возвращение результата в формате JSON
    serde_json::json!({
        "processed_data": format!("{}-от бекенда", data["text"].as_str().unwrap_or_default())
    })
}
