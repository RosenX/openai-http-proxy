use abi::{HttpService, InternalError, Url};
use async_trait::async_trait;
use feed_rs::{model::Feed, parser};

use crate::{FeedParser, FeedParserOp};

impl FeedParser {
    pub fn new() -> Self {
        Self {
            http_client: HttpService::new(),
        }
    }
}

#[async_trait]
impl FeedParserOp for FeedParser {
    async fn fetch_feed(&self, url: Url) -> Result<Feed, InternalError> {
        let data = self.http_client.get(&url).await?;
        let feed = parser::parse(data.as_bytes())?;
        Ok(feed)
    }
}
