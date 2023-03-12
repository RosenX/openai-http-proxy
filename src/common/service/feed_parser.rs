use feed_rs::{model::Feed, parser};

use crate::{common::errors::InternalError};

use super::http_service::HttpService;

pub struct FeedParser;

impl FeedParser {
    pub async fn fetch_from_url(
        http_service: &HttpService,
        url: &String,
    ) -> Result<Feed, InternalError> {
        let data = http_service.get(url).await?;
        let feed = parser::parse(data.as_bytes())?;
        Ok(feed)
    }
}
