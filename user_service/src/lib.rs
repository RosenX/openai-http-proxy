mod config;
mod user_content_manager;
mod user_feed_manager;
mod user_service;

use abi::{
    Content, ContentId, DbService, FeedProfile, UserContent, UserFeed, UserId, UserInformation,
    UserProfile,
};
use async_trait::async_trait;

#[async_trait]
pub trait UserManagerOp {
    type Error;
    async fn create(&self, user_profile: UserInformation) -> Result<UserInformation, Self::Error>;
}

struct UserFeedManager {
    db_service: DbService,
}

#[async_trait]
pub trait UserFeedManagerOp {
    type Error;
    async fn create(&self, user_feed: UserFeed) -> Result<UserFeed, Self::Error>;
    async fn query(&self, user_id: UserId) -> Result<Vec<UserFeed>, Self::Error>;
}

struct UserContentManager {
    db_service: DbService,
}

#[async_trait]
pub trait UserContentManagerOp {
    type Error;
    async fn create(&self, user_content: UserContent) -> Result<UserContent, Self::Error>;
    async fn query_latest(
        &self,
        user_id: UserId,
        content_id: ContentId,
    ) -> Result<Vec<UserContent>, Self::Error>;
}

pub struct UserService {
    user_feed_manager: UserFeedManager,
    user_content_manager: UserContentManager,
}

#[async_trait]
pub trait UserServiceApi {
    type Error;
    async fn create_user_feed(
        &self,
        user: UserProfile,
        feed: FeedProfile,
    ) -> Result<UserFeed, Self::Error>;
    async fn create_user_content(
        &self,
        user: UserProfile,
        content: Content,
    ) -> Result<UserContent, Self::Error>;
    async fn create_user_content_multiple(
        &self,
        user: UserProfile,
        content_list: Vec<Content>,
    ) -> Result<Vec<UserContent>, Self::Error>;
    async fn query_user_feed(&self, user_id: UserId) -> Result<Vec<UserFeed>, Self::Error>;
    async fn query_latest_content(
        &self,
        user_id: UserId,
        content_id: ContentId,
    ) -> Result<Vec<UserContent>, Self::Error>;
}
