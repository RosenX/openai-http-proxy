mod feed_group_manager;
mod feed_item_manager;
mod feed_manager;
mod feed_update_record_manager;
mod service;

use abi::{
    ContentPullRequest, ContentPullResponse, ContentPushRequest, ContentPushResponse, InternalError,
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
    async fn pull(&self, request: ContentPullRequest)
        -> Result<ContentPullResponse, InternalError>;

    async fn push(&self, request: ContentPushRequest)
        -> Result<ContentPushResponse, InternalError>;
}
