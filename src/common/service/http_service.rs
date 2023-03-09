use reqwest::{Client};

use crate::common::errors::InternalError;

pub async fn fetch_url(url: &str) -> Result<String, InternalError> {
    let client = Client::new();
    let resp = client.get(url).send().await?;
    Ok(resp.text().await?)
}