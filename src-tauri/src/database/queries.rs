/*
Этот файл содержит код для выполнения запросов к базе данных. 
В нем могут быть функции для выполнения SQL-запросов, извлечения данных из базы данных и 
выполнения операций CRUD.
 */

use rusqlite::{Connection, Result, Row};
use crate::database::models::User;

pub fn create_user(conn: &Connection, user: &User) -> Result<usize> {
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        &[&user.name, &user.email],
    )
}

pub fn get_user_by_id(conn: &Connection, id: i32) -> Result<User> {
    let mut stmt = conn.prepare("SELECT id, name, email FROM users WHERE id = ?1")?;
    let user_row = stmt.query_row(&[&id], User
        