use std::process::Command;
use chrono::Local;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn get_os_info() -> String {
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
