use abi::{Content, FeedProfile, HttpService, InternalError, Url};
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
    async fn fetch_feed_from_url(&self, url: &Url) -> Result<Feed, InternalError> {
        let data = self.http_client.get(url).await?;
        let feed = parser::parse(data.as_bytes())?;
        Ok(feed)
    }

    async fn fetch_feed(&self, feed_profile: FeedProfile) -> Result<Feed, InternalError> {
        self.fetch_feed_from_url(&feed_profile.url).await
    }
    async fn fetch_feed_multiple(
        &self,
        feed_list: Vec<FeedProfile>,
    ) -> Result<Vec<Feed>, abi::InternalError> {
        let mut feed_result = Vec::new();
        for feed_profile in feed_list {
            let feed = self.fetch_feed(feed_profile).await?;
            feed_result.push(feed);
        }
        Ok(feed_result)
    }

    fn parse_feed_multiple(&self, feed_list: Vec<Feed>) -> Result<Vec<Content>, InternalError> {
        let mut all_content = Vec::new();
        for feed in feed_list {
            let content_list = self.parse_feed(feed)?;
            all_content.extend(content_list)
        }
        Ok(all_content)
    }

    fn parse_feed(&self, feed: Feed) -> Result<Vec<Content>, InternalError> {
        let mut content_list = Vec::new();
        for entry in feed.entries {
            content_list.push(Content::from_entry(entry)?)
        }
        Ok(content_list)
    }
}
