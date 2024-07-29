/*
Этот файл содержит код для выполнения запросов к базе данных. 
В нем могут быть функции для выполнения SQL-запросов, извлечения данных из базы данных и 
выполнения операций CRUD.
 */

use rusqlite::{Connection, Result};

/// Функция для работы с запросами к БД SQLite
///
/// # Примеры
///
/// ```
/// use crate::database::connections;
///
/// let db = connections::get_connection().expect("Failed to get database connection");
/// connections::execute_query(&db, "CREATE TABLE IF NOT EXISTS animals (id INTEGER PRIMARY KEY, name TEXT NOT NULL);")?;
/// ```
pub fn execute_query(db: &Connection, query: &str) -> Result<()> {
    db.execute_batch(query)
}
