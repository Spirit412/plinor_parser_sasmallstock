#![allow(dead_code, unused_variables)]
mod http_client;
mod html_parser;
mod data_processor;
use chrono::{ Utc };
#[path = "../database/models.rs"]
mod models;
#[path = "../database/connection.rs"]
mod connection;

#[path = "../logging.rs"]
mod logging;

use crate::config;
use http_client::fetch_html;
use html_parser::{ parse_breeds, get_table_head, get_table_row };
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
    println!(
        "Ответ от функции header_columns # module:{}, line:{}, data:{:#?}",
        module_path!(),
        line!(),
        &header_columns.join(", ")
    );

    let table_rows: Vec<Vec<String>> = get_table_row(&html, &config::CONFIG.base_url).await.expect(
        "Ошибка получения списка строк таблицы животных"
    );

    println!(
        "Ответ от функции table_rows # module:{}, line:{}, data:{:#?}",
        module_path!(),
        line!(),
        &table_rows
    );

    let animals_data_list: Vec<models::AnimalsData> = table_rows
        .iter()
        .map(|row: &Vec<String>| models::AnimalsData {
            id: None,
            create_at: Utc::now().to_rfc3339(),
            update_at: Utc::now().to_rfc3339(),
            o: row[0].clone().to_string(),
            m: row[1].clone(),
            oo: row[2].clone(),
            mo: row[3].clone(),
            om: row[4].clone(),
            mm: row[5].clone(),
            flock_name: row[6].clone(),
            herd_book_section: row[7].clone(),
            inbreeding: row[8].clone(),
            keeper: row[9].clone(),
            owner: row[10].clone(),
            weaned_progeny: row[11].clone(),
            flocks_with_weaned_progeny: row[12].clone(),
            daughters_with_weaned_progeny: row[13].clone(),
            flocks_with_daughters_with_weaned_progeny: row[14].clone(),
            pre_wean_direct: row[15].clone(),
            pre_wean_maternal: row[16].clone(),
            wean_direct: row[17].clone(),
            wean_maternal: row[18].clone(),
            pre_wean_combined: row[19].clone(),
            wean_combined: row[20].clone(),
            post_wean_direct: row[21].clone(),
            number_of_lambs_weaned: row[22].clone(),
            total_weight_weaned: row[23].clone(),
            age_at_first_lambing: row[24].clone(),
            inter_lamb_period: row[25].clone(),
            growth_index: row[26].clone(),
            reproduction_index: row[27].clone(),
            logix_merit_index: row[28].clone(),
            dorper_logix_merit_index: row[29].clone(),
            url_animal_card: row[30].clone(),
            breed: row[31].clone(),
            parse_card: row[32].clone(),
            fdm: row[33].clone(),
            name: row[34].clone(),
            comp_no: row[35].clone(),
            id_animal: row[36].clone(),
            byear: row[37].clone(),
            inbred: row[38].clone(),
            status: row[39].clone(),
            sex: row[40].clone(),
            lmi_1: row[41].clone(),
            lmi_2: row[42].clone(),
            lmi_3: row[43].clone(),
            lmi_4: row[44].clone(),
            growth_ind_1: row[45].clone(),
            growth_ind_2: row[46].clone(),
            growth_ind_3: row[47].clone(),
            growth_ind_4: row[48].clone(),
            reprod_ind_1: row[49].clone(),
            reprod_ind_2: row[0].clone(),
            reprod_ind_3: row[0].clone(),
            reprod_ind_4: row[0].clone(),
            wean_dir_1: row[0].clone(),
            wean_dir_2: row[0].clone(),
            wean_dir_3: row[0].clone(),
            wean_dir_4: row[0].clone(),
            wean_mat_1: row[0].clone(),
            wean_mat_2: row[0].clone(),
            wean_mat_3: row[0].clone(),
            wean_mat_4: row[0].clone(),
            wean_comb_1: row[0].clone(),
            wean_comb_2: row[0].clone(),
            wean_comb_3: row[0].clone(),
            wean_comb_4: row[0].clone(),
            n_lambs_weaned_1: row[0].clone(),
            n_lambs_weaned_2: row[0].clone(),
            n_lambs_weaned_3: row[0].clone(),
            n_lambs_weaned_4: row[0].clone(),
            tww_1: row[0].clone(),
            tww_2: row[0].clone(),
            tww_3: row[0].clone(),
            tww_4: row[0].clone(),
            afl_1: row[0].clone(),
            afl_2: row[0].clone(),
            afl_3: row[0].clone(),
            afl_4: row[0].clone(),
            ilp_1: row[0].clone(),
            ilp_2: row[0].clone(),
            ilp_3: row[0].clone(),
            ilp_4: row[0].clone(),
        })
        .collect();

    // Инициализация БАЗЫ ДАННЫХ SQLite
    let conn = connection::get_connection().expect("Ошибка инициализации подключения к БД");
    models::AnimalsData::create_table(&conn).unwrap();
    println!(
        "Ответ  # module:{}, line:{}, msg:{:#?}",
        module_path!(),
        line!(),
        "Таблица animals в БД SQLite создана"
    );
    header_columns
}
