mod http_client;
mod html_parser;
mod data_processor;

use http_client::fetch_html;
use html_parser::parse_breeds;
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
    let url = "https://www.sasmallstock.com/index.php?ppd=serv_list&breed=";
    log_with_context(module_path!(), line!(), "Starting fetch_html");
    let html = fetch_html(url).await.expect("Failed to fetch HTML");
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
