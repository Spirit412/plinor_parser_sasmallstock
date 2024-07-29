#![allow(dead_code, unused_variables)]
use reqwest::Client;
use std::error::Error;

pub async fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"
        )
        .danger_accept_invalid_certs(true) // Отключение проверки SSL-сертификата
        .build()?;

    let response = client.get(url).send().await?;
    if response.status().is_success() {
        let html = response.text().await?;
        Ok(html)
    } else {
        Err(format!("Error fetching HTML: {}", response.status()).into())
    }
}
