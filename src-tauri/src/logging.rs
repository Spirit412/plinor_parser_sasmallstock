#![allow(dead_code, unused_variables)]
use log::{ info, LevelFilter };
use simple_logging;
use chrono::Local;

/// Инициализирует журналирование для приложения
///
/// # Возвращаемое значение
///
/// Возвращает `Result<(), Error>` с ошибкой, если не удалось инициализировать журналирование.
///
/// # Пример
///
/// ```
/// use log::info;
///
/// fn main() {
///     init_logging();
///
///     info!("Приложение запущено");
/// }
/// ```
pub fn init_logging() {
    simple_logging::log_to_file("app.log", LevelFilter::Info).unwrap();
}

/// Логирует сообщение с контекстом
///
/// Логирует сообщение в журнал с указанием модуля, строки и столбца кода.
///
/// # Параметры
///
/// - `module`: строка, модуль, в котором выполняется логирование
/// - `line`: целое число, номер строки, в которой выполняется логирование
/// - `message`: строка, сообщение для логирования
///
/// # Пример
///
/// ```
/// use log::info;
///
/// fn main() {
///     log_with_context(module_path!(), line!(), "Сообщение");
/// }
/// ```
/// # Примечание
///
/// ```
/// module_path!() - имя модуля, в котором выполняется логирование
/// line!() - номер строки, в которой выполняется логирование
/// ```
pub fn log_with_context(module: &str, line: u32, message: &str) {
    let now = Local::now();
    info!("[{}] [{}] [{}:{}] {}", now.format("%Y-%m-%d %H:%M:%S"), module, file!(), line, message);
}
