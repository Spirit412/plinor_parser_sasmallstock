#![allow(dead_code, unused_variables)]
use serde::Serialize;
use serde_json::{ to_string, to_string_pretty };
use std::error::Error;

use crate::logging::log_with_context;

// use super::html_parser::Breed;
use crate::controllers::html_parser::Breed;

#[derive(Serialize, Debug)]
struct BreedData {
    name: String,
    name_short: String,
}
#[allow(dead_code)]
pub async fn process_and_print_data(breeds: Vec<Breed>) -> Result<(), Box<dyn Error>> {
    log_with_context(module_path!(), line!(), "Starting process_and_print_data");
    let mut breed_data = Vec::new();

    for breed in breeds {
        breed_data.push(BreedData {
            name: breed.name,
            name_short: breed.name_short,
        });
    }

    log_with_context(module_path!(), line!(), "Parsed data");

    match to_string_pretty(&breed_data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error converting to JSON: {}", e),
    }

    Ok(())
}

pub async fn process_and_return_json(breeds: Vec<Breed>) -> Result<String, Box<dyn Error>> {
    log_with_context(module_path!(), line!(), "Starting process_and_return_json");
    let mut breed_data = Vec::new();

    for breed in breeds {
        breed_data.push(BreedData {
            name: breed.name,
            name_short: breed.name_short,
        });
    }

    log_with_context(module_path!(), line!(), "Parsed data");

    let json = to_string(&breed_data)?;
    Ok(json)
}
