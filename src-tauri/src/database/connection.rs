#![allow(dead_code, unused_variables)]
/*Этот файл содержит код, связанный с установкой и управлением соединениями с базой данных. 
В нем могут быть функции для подключения к базе данных, получения объектов соединения и обработки ошибок 
соединения.
 */

use rusqlite::{Connection, Result};
use std::path::Path;
#[path = "../config.rs"]
mod config;

/// Функция получения соединения с БД SQLite
///
/// Если файла нет, создать.
///
/// # Примеры
///
/// ```
/// use crate::database::connections;
///
/// let db = connections::get_connection().expect("Failed to get database connection");
/// ```
pub fn get_connection() -> Result<Connection> {
    let database_file_name = &config::CONFIG.database_file_name;
    let database_file_path = Path::new(database_file_name);

    if !database_file_path.exists() {
        // Create the database file if it doesn't exist
        let _ = std::fs::File::create(database_file_path);
    }

    Connection::open(database_file_path)
}
