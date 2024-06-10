use std::collections::HashMap;
use reqwest::Error;
use select::document::Document;
use select::predicate::{ Name, Predicate };

use tokio::task;

#[tauri::command]
pub async fn get_html_table_list_animals() -> String {
    // Эмулируем асинхронную операцию
    let result = task
        ::spawn_blocking(move || {
            // Здесь должна быть ваша асинхронная логика
            "<table><tr><td>Cat</td></tr><tr><td>Dog</td></tr></table>"
        }).await
        .unwrap();

    result.to_string()
}

pub async fn __get_html_table_list_animals() -> Result<(), Error> {
    // URL сайта, который вы хотите проанализировать
    let url = "tttt";

    // Загрузка HTML содержимого страницы
    let body = reqwest::get(url).await?.text().await?;

    // Создание объекта Document для парсинга HTML
    let document = Document::from(body.as_str());

    // Ассоциативный массив для хранения результатов парсинга
    let mut results = HashMap::new();

    // Пример: парсим все ссылки на странице
    for node in document.find(Name("a")) {
        // Извлечение текста и href из тега <a>
        let text = node.text();
        let href = node.attr("href").unwrap_or("");

        // Добавление результатов в ассоциативный массив
        results.insert(text.trim().to_string(), href.to_string());
    }

    // Вывод результатов
    for (key, value) in &results {
        println!("{}: {}", key, value);
    }

    Ok(())
}

pub async fn get_breeds(url: String) -> Result<(), Error> {
    // URL сайта, который вы хотите проанализировать
    // Загрузка HTML содержимого страницы
    let body = reqwest::get(url).await?.text().await?;

    // Создание объекта Document для парсинга HTML
    let document = Document::from(body.as_str());

    // Ассоциативный массив для хранения результатов парсинга
    let mut results = HashMap::new();

    // Пример: парсим все ссылки на странице
    for node in document.find(Name("a")) {
        // Извлечение текста и href из тега <a>
        let text = node.text();
        let href = node.attr("href").unwrap_or("");

        // Добавление результатов в ассоциативный массив
        results.insert(text.trim().to_string(), href.to_string());
    }

    // Вывод результатов
    for (key, value) in &results {
        println!("{}: {}", key, value);
    }

    Ok(())
}
