#![deny(unused_crate_dependencies)]

mod feed_group_manager;
mod feed_item_manager;
mod feed_manager;
mod feed_update_record_manager;
mod service;

use abi::{
    ContentPullRequest, ContentPullResponse, ContentPushRequest, ContentPushResponse, Id,
    InternalError, SubscribeFeedRequest, SubscribeFeedResponse,
};
use async_trait::async_trait;
use feed_group_manager::FeedGroupManager;
use feed_item_manager::FeedItemManager;
use feed_manager::FeedManager;
use feed_update_record_manager::FeedUpdateRecordManager;

pub struct ContentSyncService {
    feed_manager: FeedManager,
    feed_group_manager: FeedGroupManager,
    feed_item_manager: FeedItemManager,
    feed_update_record_manager: FeedUpdateRecordManager,
}

#[async_trait]
pub trait ContentSyncServiceApi {
    async fn pull(
        &self,
        user_id: Id,
        request: ContentPullRequest,
    ) -> Result<ContentPullResponse, InternalError>;

    async fn push(
        &self,
        user_id: Id,
        request: ContentPushRequest,
    ) -> Result<ContentPushResponse, InternalError>;

    async fn delete_user_content(&self, user_id: Id) -> Result<(), InternalError>;

    async fn subscribe_feed(
        &self,
        user_id: Id,
        request: SubscribeFeedRequest,
    ) -> Result<SubscribeFeedResponse, InternalError>;
}
