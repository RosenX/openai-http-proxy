use abi::{DbService, Feed, Id, InternalError};
use async_trait::async_trait;

pub struct FeedManager {
    db_service: DbService,
}

impl FeedManager {
    pub fn new(db_service: DbService) -> Self {
        Self { db_service }
    }
}

#[async_trait]
pub trait FeedManageOp {
    async fn insert(&self, user_id: Id, feed: Feed) -> Result<(), abi::InternalError>;
    async fn insert_batch(&self, user_id: Id, feeds: Vec<Feed>) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        last_feed_id: Id,
    ) -> Result<Vec<Feed>, abi::InternalError>;
}

#[async_trait]
impl FeedManageOp for FeedManager {
    async fn insert(&self, user_id: Id, feed: Feed) -> Result<(), InternalError> {
        todo!();
    }

    async fn insert_batch(&self, user_id: Id, feeds: Vec<Feed>) -> Result<(), InternalError> {
        todo!();
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        last_feed_id: Id,
    ) -> Result<Vec<Feed>, InternalError> {
        todo!();
    }
}
