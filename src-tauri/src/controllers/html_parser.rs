use scraper::{ Html, Selector };
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
