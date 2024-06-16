use log::{info, LevelFilter};
use simple_logging;
use chrono::Local;

pub fn init_logging() {
    simple_logging::log_to_file("app.log", LevelFilter::Info).unwrap();
}

pub fn log_with_context(module: &str, line: u32, message: &str) {
    let now = Local::now();
    info!("[{}] [{}] [{}:{}] {}", now.format("%Y-%m-%d %H:%M:%S"), module, file!(), line, message);
}