#![allow(dead_code, unused_variables, unused_imports)]
/*Этот файл содержит код, связанный с установкой и управлением соединениями с базой данных. 
В нем могут быть функции для подключения к базе данных, получения объектов соединения и обработки ошибок 
соединения.
 */

#[path = "../config.rs"]
mod config;

use std::fs;
use std::path::{ Path, PathBuf };
use tracing::{ info, error };
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use home::home_dir;
use tracing::field::display;

pub fn init_db() {
    if !db_file_exists() {
        create_db_file();
    }
    create_animals_table(establish_connection());
}

/// Устанавливает соединение с базой данных SQLite.
fn establish_connection() -> SqliteConnection {
    let database_url = get_db_path();
    SqliteConnection::establish(&database_url).unwrap_or_else(|_|
        panic!("Error connecting to {:#?}", database_url)
    )
}

/// Check whether the database file exists.
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

/// Create the database file.
fn create_db_file() {
    let db_path = get_db_path();
    let db_dir: &Path = Path::new(&db_path).parent().unwrap();

    if !fs::metadata(&db_dir).is_ok() {
        // Create the database file.
        fs::File::create(&db_path).expect("Failed to create database file");
        info!("Database file created at {:#?}", db_path);
    } else {
        info!("Database file already exists at {:#?}", db_path);
    }
}

/// Get the path where the database file should be located.
fn get_db_path() -> String {
    let database_file_name: String = config::CONFIG.database_file_name.clone();
    return database_file_name;
}

/// Создает таблицу `animals` в базе данных.
fn create_animals_table(mut conn: SqliteConnection) {
    // let conn: SqliteConnection = establish_connection();

    diesel
        ::sql_query(
            "CREATE TABLE IF NOT EXISTS animals (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                o TEXT,
                m TEXT,
                oo TEXT,
                mo TEXT,
                om TEXT,
                mm TEXT,
                flock_name TEXT,
                status TEXT,
                herd_book_section TEXT,
                inbreeding TEXT,
                keeper TEXT,
                owner TEXT,
                weaned_progeny TEXT,
                flocks_with_weaned_progeny TEXT,
                daughters_with_weaned_progeny TEXT,
                flocks_with_daughters_with_weaned_progeny TEXT,
                pre_wean_direct TEXT,
                pre_wean_maternal TEXT,
                wean_direct TEXT,
                wean_maternal TEXT,
                pre_wean_combined TEXT,
                wean_combined TEXT,
                post_wean_direct TEXT,
                number_of_lambs_weaned TEXT,
                total_weight_weaned TEXT,
                age_at_first_lambing TEXT,
                inter_lamb_period TEXT,
                growth_index TEXT,
                reproduction_index TEXT,
                logix_merit_index TEXT,
                dorper_logix_merit_index TEXT,
                url_animal_card TEXT,
                breed TEXT NOT NULL,
                parse_card INTEGER DEFAULT 0,
                fdm TEXT,
                name TEXT,
                comp_no TEXT,
                id_animal TEXT UNIQUE,
                byear TEXT,
                inbred TEXT,
                sex TEXT,
                lmi_1 TEXT,
                lmi_2 TEXT,
                lmi_3 TEXT,
                lmi_4 TEXT,
                growth_ind_1 TEXT,
                growth_ind_2 TEXT,
                growth_ind_3 TEXT,
                growth_ind_4 TEXT,
                reprod_ind_1 TEXT,
                reprod_ind_2 TEXT,
                reprod_ind_3 TEXT,
                reprod_ind_4 TEXT,
                wean_dir_1 TEXT,
                wean_dir_2 TEXT,
                wean_dir_3 TEXT,
                wean_dir_4 TEXT,
                wean_mat_1 TEXT,
                wean_mat_2 TEXT,
                wean_mat_3 TEXT,
                wean_mat_4 TEXT,
                wean_comb_1 TEXT,
                wean_comb_2 TEXT,
                wean_comb_3 TEXT,
                wean_comb_4 TEXT,
                n_lambs_weaned_1 TEXT,
                n_lambs_weaned_2 TEXT,
                n_lambs_weaned_3 TEXT,
                n_lambs_weaned_4 TEXT,
                tww_1 TEXT,
                tww_2 TEXT,
                tww_3 TEXT,
                tww_4 TEXT,
                afl_1 TEXT,
                afl_2 TEXT,
                afl_3 TEXT,
                afl_4 TEXT,
                ilp_1 TEXT,
                ilp_2 TEXT,
                ilp_3 TEXT,
                ilp_4 TEXT,
                create_at TEXT NOT NULL,
                update_at TEXT NOT NULL
            )",
        )
        .execute(&mut conn)
        .expect("Error creating table");

    info!("Table 'users' created successfully");
}
