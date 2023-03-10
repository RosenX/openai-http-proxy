use crate::common::errors::InternalError;

use super::http_service::HttpService;

pub struct FeedParser;

impl FeedParser {
    pub async fn fetch_from_url(
        http_service: &HttpService,
        url: &String,
    ) -> Result<String, InternalError> {
        let data = http_service.get(url).await?;
        Ok(data)
    }
}
