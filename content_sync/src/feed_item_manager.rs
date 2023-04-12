use abi::{DbService, FeedItem, Id, InternalError};
use async_trait::async_trait;

pub struct FeedItemManager {
    db_service: DbService,
}

impl FeedItemManager {
    pub fn new(db_service: DbService) -> Self {
        Self { db_service }
    }
}

#[async_trait]
pub trait FeedItemManageOp {
    async fn insert(&self, user_id: Id, feed_item: FeedItem) -> Result<(), abi::InternalError>;
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_items: Vec<FeedItem>,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        last_item_id: Id,
    ) -> Result<Vec<FeedItem>, abi::InternalError>;
}

#[async_trait]
impl FeedItemManageOp for FeedItemManager {
    async fn insert(&self, user_id: Id, feed_item: FeedItem) -> Result<(), InternalError> {
        todo!();
    }

    async fn insert_batch(
        &self,
        user_id: Id,
        feed_items: Vec<FeedItem>,
    ) -> Result<(), InternalError> {
        todo!();
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        last_item_id: Id,
    ) -> Result<Vec<FeedItem>, InternalError> {
        todo!();
    }
}
