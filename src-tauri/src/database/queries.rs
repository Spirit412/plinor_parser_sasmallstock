#![allow(dead_code, unused_variables)]
/*
Этот файл содержит код для выполнения запросов к базе данных. 
В нем могут быть функции для выполнения SQL-запросов, извлечения данных из базы данных и 
выполнения операций CRUD.
 */

use diesel::sqlite::SqliteConnection;
use super::models::{AnimalData, NewAnimal};


// pub fn create_animal(conn: &mut SqliteConnection, new_animal: &NewAnimal) -> AnimalData {
//     use crate::database::schema::animals;

//     diesel::insert_into(animals::table)
//         .values(&new_animal)
//         .returning(AnimalData::as_returning())
//         .get_result(conn)
//         .expect("Error saving new animal")
// }