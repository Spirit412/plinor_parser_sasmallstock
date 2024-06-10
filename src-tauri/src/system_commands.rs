use chrono::Local;
use std::env;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn get_os_info() -> String {
    let cur_os = env::consts::OS;
    let info = os_info::get();
    format!(
        "{}, Type: {:#?} Version: {:#?} Bitness: {:#?} Architecture: {:#?}",
        cur_os,
        info.os_type(),
        info.version(),
        info.bitness(),
        info.architecture()
    )
}

#[tauri::command]
pub fn get_current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
