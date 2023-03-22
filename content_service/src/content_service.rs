use std::collections::HashSet;

use abi::{Content, DbService, FeedContentResponse, FeedProfile};
use async_trait::async_trait;
use tokio::time;

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

    pub fn start_fetch_content(self) {
        tokio::spawn(async move {
            //todo 增加异常处理
            let mut interval = tokio::time::interval(time::Duration::from_secs(60));
            loop {
                let exist_content = self
                    .content_manager
                    .query_all_md5()
                    .await
                    .map_err(|err| println!("{}", err))
                    .unwrap();

                let mut md5_hash_set = HashSet::new();
                for item in exist_content {
                    md5_hash_set.insert(item.md5);
                }

                let feed_list = self.feed_manager.query_all().await.unwrap();

                let feed_list = self
                    .feed_parser
                    .fetch_feed_multiple(feed_list)
                    .await
                    .map_err(|err| println!("{}", err))
                    .unwrap();

                let content_list = self
                    .feed_parser
                    .parse_feed_multiple(feed_list)
                    .map_err(|err| println!("{}", err))
                    .unwrap();

                let mut filter_content_list = Vec::new();
                for content in content_list {
                    if !md5_hash_set.contains(&content.md5) {
                        filter_content_list.push(content);
                    }
                }

                self.content_manager
                    .create_multiple(filter_content_list)
                    .await
                    .map_err(|err| println!("{}", err))
                    .unwrap();

                interval.tick().await;
            }
        });
    }
}

#[async_trait]
impl ContentServiceApi for ContentService {
    async fn create_feed(
        &self,
        feed_request: abi::CreateFeedRequest,
    ) -> Result<abi::FeedContentResponse, abi::InternalError> {
        let url = feed_request.url;
        let feed = self.feed_parser.fetch_feed_from_url(&url).await?;

        let feed_profile = FeedProfile::new(&feed, url);
        let feed_profile = self.feed_manager.create(feed_profile).await?;

        let mut content_list = Vec::with_capacity(feed.entries.len());
        for entry in feed.entries {
            content_list.push(Content::from_entry(entry)?);
        }

        let contents = self.content_manager.create_multiple(content_list).await?;

        Ok(FeedContentResponse {
            feed_profile,
            contents,
        })
    }
}
