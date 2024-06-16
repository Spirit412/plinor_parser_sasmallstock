use reqwest::Error;
use reqwest::blocking;
use tokio::task;
use std::fs::File;
use std::io::prelude::*;
use reqwest::Client;
use reqwest::StatusCode;

use crate::enums;
let mut retry_attempts = 0;
pub mod read_file {
    pub fn read_html_file(filename: String) -> String {
        let mut f = File::open(filename).expect("Файл не найден!");
        let mut content: String = String::new();
        f.read_to_string(&mut content).expect("Что-то с файлом. Не могу прочитать");
        print!("Content file: \n {}", &content);
        return content;
    }
}

#[tauri::command]
pub async fn __get_html_table_list_animals() -> String {
    // Эмулируем асинхронную операцию
    let result = task
        ::spawn_blocking(move || {
            // Здесь должна быть ваша асинхронная логика
            "<table><tr><td>Cat</td></tr><tr><td>Dog</td></tr></table>"
        }).await
        .unwrap();

    result.to_string()
}

#[tauri::command]
pub async fn get_html_table_list_animals(
    sop_brd: enums::Breed,
    sop_sex: &str,
    soek_lim: u32
) -> Result<String, Error> {
    let brd;

    match sop_brd {
        enums::Breed::Dorper => {
            brd = "dorper".to_string();
        }
        enums::Breed::IleDeFrance => {
            brd = "ilDeFrance".to_string();
        }
    }

    let url = format!(
        "https://www.sasmallstock.com/index.php?ppd=serv_list_sql&breed=&sop_brd={:#?}&sop_reg=A&sop_sex={:#?}&soek_lim={:#?}&sop_gt=ALL&sop_obj=lmi&filter1=IND&age1_sig=%3E%3D&age1_val=&age2_sig=%3C%3D&age2_val=&lmi1_sig=&lmi1_sigv=&lmi2_sig=&lmi2_sigv=&ngi1_sig=&ngi1_sigv=&ngi2_sig=&ngi2_sigv=&ori1_sig=&ori1_sigv=&ori2_sig=&ori2_sigv=&wean_dir1_sig=&wean_dir1_sigv=&wean_dir2_sig=&wean_dir2_sigv=&wean_dirA_sig=&wean_dirA_sigv=&wean_mat1_sig=&wean_mat1_sigv=&wean_mat2_sig=&wean_mat2_sigv=&wean_matA_sig=&wean_matA_sigv=&wean_comb1_sig=&wean_comb1_sigv=&wean_comb2_sig=&wean_comb2_sigv=&post_dir1_sig=&post_dir1_sigv=&post_dir2_sig=&post_dir2_sigv=&post_dirA_sig=&post_dirA_sigv=&n_weaned1_sig=&n_weaned1_sigv=&n_weaned2_sig=&n_weaned2_sigv=&n_weanedA_sig=&n_weanedA_sigv=&afb1_sig=&afb1_sigv=&afb2_sig=&afb2_sigv=&afbA_sig=&afbA_sigv=&ilp1_sig=&ilp1_sigv=&ilp2_sig=&ilp2_sigv=&ilpA_sig=&ilpA_sigv=",
        brd,
        sop_sex.to_string(),
        soek_lim.to_string()
    );

    reqwest
        ::get(url).await?
        .text().await
        .map_err(|err| err.into())
}

const USER_AGENT: &str = "Mozilla/5.0 (Linux x86_64; rv:115.0) Gecko/20100101 Firefox/115.0";

#[tauri::command]
pub async fn get_html_breeds(url: &str) -> Result<(), String> {
    let ctx: Client = Client::builder().user_agent(USER_AGENT).build().unwrap();

    let res: reqwest::Response = match ctx.get(url).send().await {
        Ok(res) => res,
        Err(e) => return Err(format!("Error while attempting to send HTTP request: {e}")),
    };

    let res: String = match res.text().await {
        Ok(res) => res,
        Err(e) => return Err(format!("Error while attempting to get the HTTP body: {e}")),
    };

    Ok(())
}