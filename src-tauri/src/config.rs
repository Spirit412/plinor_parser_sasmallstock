use std::env;
use serde::Deserialize;
use validator::{Validate};
use lazy_static::lazy_static;


#[derive(Debug, Validate, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    #[validate(length(min = 1))]
    pub database_file_name: String,
    #[validate(length(min = 1))]
    pub name_table: String,
    #[validate(url)]
    pub base_url: String,
    #[validate(range(min = 1))]
    pub limit: usize,
    pub testing: bool,
    #[validate(length(min = 1))]
    pub breed: String,
}

fn init_config() -> Config {
    Config {
        database_file_name: env::var("DATABASE_FILE_NAME").unwrap_or_else(|_| "database.db".to_string()),
        name_table: env::var("NAME_TABLE").unwrap_or_else(|_| "animals".to_string()),
        base_url: env::var("BASE_URL").unwrap_or_else(|_| "https://www.sasmallstock.com/".to_string()),
        limit: env::var("LIMIT").unwrap_or_else(|_| "12000".to_string()).parse().unwrap_or(12000),
        testing: env::var("TESTING").unwrap_or_else(|_| "false".to_string()).parse().unwrap_or(false),
        breed: env::var("BREED").unwrap_or_else(|_| "ile de france".to_string()),
    }
}

lazy_static! {
    pub static ref CONFIG: Config = init_config();
}