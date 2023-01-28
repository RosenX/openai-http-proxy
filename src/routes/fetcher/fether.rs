use anyhow::Result;
use reqwest::{Url, Client};

pub async fn fetch_uri(url: &str) -> Result<String> {
    let client = Client::new();
    let resp = client.get(url).send().await?;
    Ok(resp.text().await?)
}