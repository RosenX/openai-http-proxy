use abi::{Content, CreateFeedResponse, DbPool};
use async_trait::async_trait;

use crate::{
    ContentManageOp, ContentManager, ContentService, ContentServiceApi, FeedManageOp, FeedManager,
    FeedParser, FeedParserOp,
};

impl ContentService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            feed_manager: FeedManager::new(pool.clone()),
            content_manager: ContentManager::new(pool),
            feed_parser: FeedParser::new(),
        }
    }
}

#[async_trait]
impl ContentServiceApi for ContentService {
    async fn create_feed(
        &self,
        feed_request: abi::CreateFeedRequest,
    ) -> Result<abi::CreateFeedResponse, abi::InternalError> {
        let feed = self.feed_parser.fetch_feed(feed_request.url).await?;

        let feed_profile = feed.clone().into();
        let feed_profile = self.feed_manager.create(feed_profile).await?;

        let content = feed
            .entries
            .iter()
            .map(|entry| Content::from(entry.to_owned()))
            .collect();

        let content = self.content_manager.create_multiple(content).await?;

        Ok(CreateFeedResponse {
            feed_profile,
            content,
        })
    }
}
