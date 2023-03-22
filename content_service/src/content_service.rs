use abi::{Content, DbService, FeedContentResponse};
use async_trait::async_trait;

use crate::{
    ContentManageOp, ContentManager, ContentService, ContentServiceApi, FeedManageOp, FeedManager,
    FeedParser, FeedParserOp,
};

impl ContentService {
    pub fn new(db_service: DbService) -> Self {
        Self {
            feed_manager: FeedManager::new(db_service.clone()),
            content_manager: ContentManager::new(db_service),
            feed_parser: FeedParser::new(),
        }
    }
}

#[async_trait]
impl ContentServiceApi for ContentService {
    async fn create_feed(
        &self,
        feed_request: abi::CreateFeedRequest,
    ) -> Result<abi::FeedContentResponse, abi::InternalError> {
        let feed = self.feed_parser.fetch_feed(feed_request.url).await?;

        let feed_profile = feed.clone().into();
        let feed_profile = self.feed_manager.create(feed_profile).await?;

        let content = feed
            .entries
            .iter()
            .map(|entry| Content::from(entry.to_owned()))
            .collect();

        let content = self.content_manager.create_multiple(content).await?;

        Ok(FeedContentResponse {
            feed_profile,
            content,
        })
    }
}
