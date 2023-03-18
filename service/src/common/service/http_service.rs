use reqwest::Client;

use crate::common::errors::InternalError;

pub struct HttpService {
    client: Client,
}

impl HttpService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    pub async fn get(&self, url: &String) -> Result<String, InternalError> {
        let resp = self.client.get(url).send().await?;
        Ok(resp.text().await?)
    }
}
