use abi::{Content, DbService, FeedProfile, Id, InternalError, UserContent, UserFeed, UserProfile};
use async_trait::async_trait;

use crate::{
    UserContentManager, UserContentManagerOp, UserFeedManager, UserFeedManagerOp, UserService,
    UserServiceApi, UserServiceConfig,
};

impl UserService {
    pub fn new(db_service: DbService, config: UserServiceConfig) -> Self {
        Self {
            user_feed_manager: UserFeedManager::new(db_service.clone()),
            user_content_manager: UserContentManager::new(db_service),
            config,
        }
    }
}

#[async_trait]
impl UserServiceApi for UserService {
    type Error = InternalError;

    async fn create_user_feed(
        &self,
        user: UserProfile,
        feed: FeedProfile,
    ) -> Result<UserFeed, Self::Error> {
        let user_feed = UserFeed::new(user, feed);
        let user_feed = self.user_feed_manager.create(user_feed).await?;
        Ok(user_feed)
    }

    async fn create_user_content(
        &self,
        user: UserProfile,
        content: Content,
    ) -> Result<UserContent, Self::Error> {
        let user_content = UserContent::new(user, content);
        let user_content = self.user_content_manager.create(user_content).await?;
        Ok(user_content)
    }

    async fn create_user_content_multiple(
        &self,
        user: UserProfile,
        content_list: Vec<Content>,
    ) -> Result<Vec<UserContent>, Self::Error> {
        let mut user_content_list = Vec::with_capacity(content_list.len());
        for content in content_list {
            let user_content = self.create_user_content(user.clone(), content).await?;
            user_content_list.push(user_content);
        }
        Ok(user_content_list)
    }

    async fn query_user_feed(&self, user_id: Id) -> Result<Vec<UserFeed>, Self::Error> {
        let user_feed_list = self.user_feed_manager.query(user_id).await?;
        Ok(user_feed_list)
    }

    async fn query_latest_content(
        &self,
        user_id: Id,
        content_id: Id,
        size: i32,
    ) -> Result<Vec<UserContent>, Self::Error> {
        let size = std::cmp::min(size, self.config.max_page_size);
        let content_list = self
            .user_content_manager
            .query_latest(user_id, content_id, size)
            .await?;
        Ok(content_list)
    }

    async fn query_old_content(
        &self,
        user_id: Id,
        content_id: Id,
        size: i32,
    ) -> Result<Vec<UserContent>, Self::Error> {
        let size = std::cmp::min(size, self.config.max_page_size);
        let content_list = self
            .user_content_manager
            .query_old(user_id, content_id, size)
            .await?;
        Ok(content_list)
    }
}
