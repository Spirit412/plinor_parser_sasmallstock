#![allow(dead_code, unused_variables, unused_imports)]
mod http_client;
mod html_parser;
mod data_processor;
use chrono::{ DateTime, TimeZone, Utc };
#[path = "../database/models.rs"]
mod models;
use models::AnimalData;
#[path = "../database/db.rs"]
mod db;

#[path = "../logging.rs"]
mod logging;

use std::collections::HashMap;
use std::vec;

use crate::config;
use http_client::fetch_html;
use html_parser::{ parse_breeds, get_table_head, get_table_row };
use data_processor::process_and_return_json;
use std::process::Command;
use chrono::Local;
use tracing::{ info, error };

#[allow(dead_code)]
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
#[tauri::command]
pub async fn get_os_info() -> String {
    let os_release = Command::new("cat")
        .arg("/etc/os-release")
        .output()
        .expect("Failed to execute command");

    let os_info = String::from_utf8_lossy(&os_release.stdout);

    let mut name = String::new();
    let mut version = String::new();

    for line in os_info.lines() {
        if line.starts_with("NAME=") {
            name = line.split_once('=').unwrap().1.trim().to_string();
        } else if line.starts_with("VERSION_ID=") {
            version = line.split_once('=').unwrap().1.trim().to_string();
        }
    }

    format!("{} {}", name, version)
}

#[tauri::command]
pub fn get_current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tokio::main]
#[tauri::command]
pub async fn get_breeds() -> String {
    let base_url = &config::CONFIG.base_url;

    info!("Получить список пород сайта породы для обновления списка");
    let url = base_url.to_string() + "index.php?ppd=serv_list&breed=";
    info!("Starting fetch_html");
    let html = fetch_html(&url).await.expect("Failed to fetch HTML");
    info!("HTML fetched, starting parse_breeds");
    let breeds = parse_breeds(&html).await.expect("Failed to parse breeds");
    info!("Breeds parsed, starting process_and_return_json");
    let result = process_and_return_json(breeds).await;
    info!("Data processed and return json");

    match result {
        Ok(json) => json,
        Err(_) => "{}".to_string(),
    }
}

fn create_dict_list(
    header_columns: &[String],
    table_rows: &[Vec<String>]
) -> Vec<HashMap<String, String>> {
    let mut result: Vec<HashMap<String, String>> = Vec::new();

    for row in table_rows {
        let mut dict: HashMap<String, String> = HashMap::new();

        for (index, column) in header_columns.iter().enumerate() {
            dict.insert(column.clone(), row[index].clone());
        }

        result.push(dict);
    }

    result
}


#[tokio::main]
#[tauri::command]
pub async fn set_animals_to_db(
    sop_brd: &str, // DOP для Dorper и ILE для Ile de France
    sop_sex: &str,
    soek_lim: &str
) -> Vec<String> {
    let base_url = &config::CONFIG.base_url;

    /*     Запись данных по животным из таблицы списка животных в БД
    # Получаем страницу со списком животных
    # Парсим заголовок таблицы животных, получая список столбцов => header_columns
    # На основе списка создаём таблицу животных в БД SQLite. Для указанной пароды=sop_brd. Поля таблицы из header_columns. Тип String
    # Парсим каждую строку таблицы в асоциативный массив, где ключ из header_columns. Каждую строку записываем в БД
    */

    info!("Получаем страницу со списком животных. LIMIT:{}, BREED:{},", soek_lim, sop_brd);
    let url: String =
        base_url.to_string() +
        &format!(
            "index.php?ppd=serv_list_sql&breed=&sop_brd={}&sop_reg=A&sop_sex={}&soek_lim={}&sop_gt=ALL&sop_obj=lmi&filter1=IND&age1_sig=%3E%3D&age1_val=&age2_sig=%3C%3D&age2_val=&lmi1_sig=&lmi1_sigv=&lmi2_sig=&lmi2_sigv=&ngi1_sig=&ngi1_sigv=&ngi2_sig=&ngi2_sigv=&ori1_sig=&ori1_sigv=&ori2_sig=&ori2_sigv=&wean_dir1_sig=&wean_dir1_sigv=&wean_dir2_sig=&wean_dir2_sigv=&wean_dirA_sig=&wean_dirA_sigv=&wean_mat1_sig=&wean_mat1_sigv=&wean_mat2_sig=&wean_mat2_sigv=&wean_matA_sig=&wean_matA_sigv=&wean_comb1_sig=&wean_comb1_sigv=&wean_comb2_sig=&wean_comb2_sigv=&post_dir1_sig=&post_dir1_sigv=&post_dir2_sig=&post_dir2_sigv=&post_dirA_sig=&post_dirA_sigv=&n_weaned1_sig=&n_weaned1_sigv=&n_weaned2_sig=&n_weaned2_sigv=&n_weanedA_sig=&n_weanedA_sigv=&afb1_sig=&afb1_sigv=&afb2_sig=&afb2_sigv=&afbA_sig=&afbA_sigv=&ilp1_sig=&ilp1_sigv=&ilp2_sig=&ilp2_sigv=&ilpA_sig=&ilpA_sigv",
            sop_brd,
            sop_sex,
            soek_lim
        );

    info!("Starting fetch_html");
    let html: String = fetch_html(&url).await.expect(
        "Ошибка получения страницы HTML с таблицей животных"
    );
    info!("Парсим заголовок таблицы животных, получая список столбцов");
    let header_columns: Vec<String> = get_table_head(&html).await.expect(
        "Ошибка получения заголовока таблицы животных, получая список столбцов"
    );

    let table_rows: Vec<Vec<String>> = get_table_row(&html, &config::CONFIG.base_url).await.expect(
        "Ошибка получения списка строк таблицы животных"
    );

    let mut dictionary: Vec<HashMap<String, String>> = create_dict_list(
        &header_columns,
        &table_rows
    );

    // Добавим breed в словарь.
    dictionary.iter_mut().for_each(|dict| {
        dict.insert("breed".to_string(), sop_brd.to_string());
    });

    // let result: Vec<AnimalData> = dictionary
    //     .iter()
    //     .map(|x| AnimalData::new(&x).expect("Ошибка создания AnimalData"))
    //     .collect();

    // println!("{:#?}", &result);

    // Инициализация БАЗЫ ДАННЫХ SQLite
    // let conn = db::establish_db_connection().expect("Ошибка инициализации подключения к БД");

    header_columns
}
