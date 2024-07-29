mod http_client;
mod html_parser;
mod data_processor;

use crate::config;
use http_client::fetch_html;
use html_parser::{ parse_breeds, get_table_head };
use data_processor::process_and_return_json;
use std::process::Command;
use chrono::Local;
use crate::logging::log_with_context;

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

    log_with_context(
        module_path!(),
        line!(),
        "Получить список пород сайта породы для обновления списка"
    );
    let url = base_url.to_string() + "index.php?ppd=serv_list&breed=";
    log_with_context(module_path!(), line!(), "Starting fetch_html");
    let html = fetch_html(&url).await.expect("Failed to fetch HTML");
    log_with_context(module_path!(), line!(), "HTML fetched, starting parse_breeds");
    let breeds = parse_breeds(&html).await.expect("Failed to parse breeds");
    log_with_context(module_path!(), line!(), "Breeds parsed, starting process_and_return_json");
    let result = process_and_return_json(breeds).await;
    log_with_context(module_path!(), line!(), "Data processed and return json");

    match result {
        Ok(json) => json,
        Err(_) => "{}".to_string(),
    }
}

/// Записывает данные по животным из таблицы списка животных в БД
///
/// # Параметры
///
/// - `sop_brd`: строка, определяющая породу животного (DOP для Dorper или ILE для Ile de France)
/// - `sop_sex`: строка, определяющая пол животного (ALL для обоих полов или M для мужчин или F для самок)
/// - `soek_lim`: строка, определяющая количество животных для записи в БД (по умолчанию "10")
///
/// # Пример
///
/// ```
/// use tauri::command;
///
/// #[command]
/// async fn set_animals_to_db() {
///     let result = set_animals_to_db("DOP", "ALL", "10").await;
///     println!("{:?}", result);
/// }
/// ```
///
/// # Возвращаемое значение
///
/// Возвращает вектор строк с результатами записи в БД.
///
/// # Ошибки
///
/// Возвращает пустой вектор, если произошла ошибка при записи в БД.
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

    log_with_context(
        module_path!(),
        line!(),
        &format!("Получаем страницу со списком животных. LIMIT:{}, BREED:{},", soek_lim, sop_brd)
    );
    let url: String =
        base_url.to_string() +
        &format!(
            "index.php?ppd=serv_list_sql&breed=&sop_brd={}&sop_reg=A&sop_sex={}&soek_lim={}&sop_gt=ALL&sop_obj=lmi&filter1=IND&age1_sig=%3E%3D&age1_val=&age2_sig=%3C%3D&age2_val=&lmi1_sig=&lmi1_sigv=&lmi2_sig=&lmi2_sigv=&ngi1_sig=&ngi1_sigv=&ngi2_sig=&ngi2_sigv=&ori1_sig=&ori1_sigv=&ori2_sig=&ori2_sigv=&wean_dir1_sig=&wean_dir1_sigv=&wean_dir2_sig=&wean_dir2_sigv=&wean_dirA_sig=&wean_dirA_sigv=&wean_mat1_sig=&wean_mat1_sigv=&wean_mat2_sig=&wean_mat2_sigv=&wean_matA_sig=&wean_matA_sigv=&wean_comb1_sig=&wean_comb1_sigv=&wean_comb2_sig=&wean_comb2_sigv=&post_dir1_sig=&post_dir1_sigv=&post_dir2_sig=&post_dir2_sigv=&post_dirA_sig=&post_dirA_sigv=&n_weaned1_sig=&n_weaned1_sigv=&n_weaned2_sig=&n_weaned2_sigv=&n_weanedA_sig=&n_weanedA_sigv=&afb1_sig=&afb1_sigv=&afb2_sig=&afb2_sigv=&afbA_sig=&afbA_sigv=&ilp1_sig=&ilp1_sigv=&ilp2_sig=&ilp2_sigv=&ilpA_sig=&ilpA_sigv",
            sop_brd,
            sop_sex,
            soek_lim
        );

    log_with_context(module_path!(), line!(), "Starting fetch_html");
    let html: String = fetch_html(&url).await.expect(
        "Ошибка получения страницы HTML с таблицей животных"
    );
    log_with_context(
        module_path!(),
        line!(),
        "Парсим заголовок таблицы животных, получая список столбцов"
    );
    let header_columns: Vec<String> = get_table_head(&html).await.expect(
        "Ошибка получения заголовока таблицы животных, получая список столбцов"
    );
    println!("Ответ от функции get_table_head: {:#?}, ", header_columns);
    header_columns
}
