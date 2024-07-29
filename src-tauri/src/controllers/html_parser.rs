use scraper::{ ElementRef, Html, Selector };
use std::error::Error;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Breed {
    pub name: String,
    pub name_short: String,
}

#[allow(dead_code)]
impl Breed {
    // Метод для поиска name по name_short без учета регистра
    pub fn find_name_by_short(&self, short: &str) -> Option<&str> {
        if self.name_short.eq_ignore_ascii_case(short) { Some(&self.name) } else { None }
    }

    // Метод для поиска name_short по name без учета регистра
    pub fn find_short_by_name(&self, name: &str) -> Option<&str> {
        if self.name.eq_ignore_ascii_case(name) { Some(&self.name_short) } else { None }
    }
}

pub async fn parse_breeds(html: &str) -> Result<Vec<Breed>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("select#soek_brd option").unwrap();
    let mut breeds = Vec::new();

    for element in document.select(&selector) {
        if let (Some(value), Some(name)) = (element.value().attr("value"), element.text().next()) {
            breeds.push(Breed {
                name: name.to_string(),
                name_short: value.to_string(),
            });
        }
    }

    Ok(breeds)
}

pub async fn parse_rows_table_animals(html: &str) -> Result<Vec<Breed>, Box<dyn Error>> {
    let document = Html::parse_document(html);

    let selector = Selector::parse("select#soek_brd option").unwrap();
    let mut breeds = Vec::new();

    for element in document.select(&selector) {
        if let (Some(value), Some(name)) = (element.value().attr("value"), element.text().next()) {
            breeds.push(Breed {
                name: name.to_string(),
                name_short: value.to_string(),
            });
        }
    }

    Ok(breeds)
}

pub async fn get_table_head(
    table_head_html: &str
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let html: Html = Html::parse_document(table_head_html);
    let table_head_selector = Selector::parse("thead").unwrap();
    let mut table_head = html.select(&table_head_selector);

    if let Some(thead) = table_head.next() {
        let tr_selector = Selector::parse("tr").unwrap();
        if let Some(tr) = thead.select(&tr_selector).next() {
            let mut result_table_head = vec!["url_animal_card".to_string()];
            let th_selector = Selector::parse("th").unwrap(); // Получили заголовок таблицы
            // Пройдёмся по каждлому заголовку столбца
            for cell_head in tr.select(&th_selector) {
                let rows: &String = &cell_head.text().collect::<Vec<_>>().join(",");
                let rows_clear: Vec<String> = rows
                    .split(",")
                    .map(|x| x.trim().to_lowercase())
                    .filter(|x| !x.is_empty())
                    .collect();
                let rows: Vec<String> = rows_clear
                    .iter()
                    .map(|x| x.replace(".", "").replace(" ", "_"))
                    .map(|x| if x == "id" { "id_animal".to_string() } else { x.to_string() })
                    .collect();
                if rows.len() < 4 {
                    let mut rows_extended = vec![];
                    for i in 1..=4 {
                        rows_extended.push(format!("{}_{}", rows[0], i));
                    }
                    result_table_head.extend(rows_extended);
                } else {
                    result_table_head.extend(rows);
                }
            }
            Ok(result_table_head)
        } else {
            Err("No <tr> found in <thead>".into())
        }
    } else {
        Err("Заголовок таблицы не найден".into())
    }
}
