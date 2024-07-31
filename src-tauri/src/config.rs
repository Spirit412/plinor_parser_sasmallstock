#![allow(dead_code, unused_variables, unused_imports)]
use std::env;
use serde::Deserialize;
use validator::{ Validate };
use lazy_static::lazy_static;

/// Структура конфигурации приложения
///
/// Содержит значения, определенные в переменных окружения или имеющие значения по умолчанию.
///
/// # Поля структуры
///
/// - `database_file_name`: строка, имя файла базы данных (по умолчанию "database.db")
///     - Определяет имя файла базы данных, в котором хранятся данные приложения.
///     - Значение по умолчанию: "database.db"
///
/// - `name_table`: строка, имя таблицы в базе данных (по умолчанию "animals")
///     - Определяет имя таблицы в базе данных, в которой хранятся данные приложения.
///     - Значение по умолчанию: "animals"
///
/// - `base_url`: строка, базовый URL (по умолчанию `<https://www.sasmallstock.com/>`)
///     - Определяет базовый URL, с которого берутся данные для приложения.
///     - Значение по умолчанию: `<https://www.sasmallstock.com/>`
///
/// - `limit`: целое число, лимит значений для записи в базу данных (по умолчанию 12000)
///     - Определяет максимальное количество записей, которые могут быть добавлены в базу данных.
///     - Значение по умолчанию: 12000
///
/// - `testing`: булево значение, определяет, используется ли тестовая база данных (по умолчанию false)
///     - Определяет, используется ли тестовая база данных или основная база данных.
///     - Значение по умолчанию: false
///
/// - `breed`: строка, определяет породу животных (по умолчанию "ile de france")
///     - Определяет породу животных, которые будут использоваться в приложении.
///     - Значение по умолчанию: "ile de france"
///
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
/// Инициализирует конфигурацию из переменных окружения
///
/// # Возвращаемое значение
///
/// Возвращает экземпляр структуры `Config`, содержащий значения из переменных окружения.
/// Если значение переменной окружения отсутствует или не соответствует ожидаемому типу,
/// используется значение по умолчанию.
///
fn init_config() -> Config {
    Config {
        database_file_name: env
            ::var("DATABASE_FILE_NAME")
            .unwrap_or_else(|_| "database.db".to_string()),
        name_table: env::var("NAME_TABLE").unwrap_or_else(|_| "animals".to_string()),
        base_url: env
            ::var("BASE_URL")
            .unwrap_or_else(|_| "https://www.sasmallstock.com/".to_string()),
        limit: env
            ::var("LIMIT")
            .unwrap_or_else(|_| "12000".to_string())
            .parse()
            .unwrap_or(12000),
        testing: env
            ::var("TESTING")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false),
        breed: env::var("BREED").unwrap_or_else(|_| "ile de france".to_string()),
    }
}

lazy_static! {
    pub static ref CONFIG: Config = init_config();
}
