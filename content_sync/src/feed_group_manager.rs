use abi::{DbService, FeedGroup, Id, InternalError};
use async_trait::async_trait;

pub struct FeedGroupManager {
    db_service: DbService,
}

impl FeedGroupManager {
    pub fn new(db_service: DbService) -> Self {
        Self { db_service }
    }
}

#[async_trait]
pub trait FeedGroupManageOp {
    async fn insert(&self, user_id: Id, feed_group: FeedGroup) -> Result<(), abi::InternalError>;
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_groups: Vec<FeedGroup>,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        last_feed_group_id: Id,
    ) -> Result<Vec<FeedGroup>, abi::InternalError>;
}

#[async_trait]
impl FeedGroupManageOp for FeedGroupManager {
    async fn insert(&self, user_id: Id, feed_group: FeedGroup) -> Result<(), InternalError> {
        todo!();
    }

    async fn insert_batch(
        &self,
        user_id: Id,
        feed_groups: Vec<FeedGroup>,
    ) -> Result<(), InternalError> {
        todo!();
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        last_feed_group_id: Id,
    ) -> Result<Vec<FeedGroup>, InternalError> {
        todo!();
    }
}
