mod config;
mod user_content_manager;
mod user_feed_manager;
mod user_manager;
mod user_service;

use abi::{DbPool, RegisterReq, UserFeed, UserInformation, UserPost, UserProfile};
use async_trait::async_trait;
use config::ServiceConfig;

struct UserManager {
    pool: DbPool,
}

#[async_trait]
pub trait UserManagerOp {
    type Error;
    async fn create(&self, user_profile: UserInformation) -> Result<UserInformation, Self::Error>;
}

struct UserFeedManager {
    pool: DbPool,
}

#[async_trait]
pub trait UserFeedManagerOp {
    type Error;
    async fn create(&self, user_feed: UserFeed) -> Result<UserFeed, Self::Error>;
}

struct UserContentManager {
    pool: DbPool,
}

#[async_trait]
pub trait UserContentManagerOp {
    type Error;
    async fn create(&self, user_content: UserPost) -> Result<UserPost, Self::Error>;
}

pub struct UserService {
    user_manager: UserManager,
    user_feed_manager: UserFeedManager,
    user_content_manager: UserContentManager,
    config: ServiceConfig,
}

#[async_trait]
pub trait UserServiceApi {
    type Error;
    async fn register_by_email(&self, request: RegisterReq) -> Result<UserProfile, Self::Error>;
}
