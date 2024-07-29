/*Этот файл содержит код, связанный с установкой и управлением соединениями с базой данных. 
В нем могут быть функции для подключения к базе данных, получения объектов соединения и обработки ошибок 
соединения.
 */

use rusqlite::{ Connection, Result };

pub fn establish_connection() -> Result<Connection> {
    let database_path = "/database.db";
    Connection::open(database_path)
}
