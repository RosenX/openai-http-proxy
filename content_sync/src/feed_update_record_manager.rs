use abi::{DbService, FeedUpdateRecord, Id, InternalError};
use async_trait::async_trait;

pub struct FeedUpdateRecordManager {
    db_service: DbService,
}

impl FeedUpdateRecordManager {
    pub fn new(db_service: DbService) -> Self {
        Self { db_service }
    }
}

#[async_trait]
pub trait FeedUpdateRecordManageOp {
    async fn insert(
        &self,
        user_id: Id,
        feed_update_record: FeedUpdateRecord,
    ) -> Result<(), abi::InternalError>;
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_update_records: Vec<FeedUpdateRecord>,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        last_feed_update_record_id: Id,
    ) -> Result<Vec<FeedUpdateRecord>, abi::InternalError>;
}

#[async_trait]
impl FeedUpdateRecordManageOp for FeedUpdateRecordManager {
    async fn insert(
        &self,
        user_id: Id,
        feed_update_record: FeedUpdateRecord,
    ) -> Result<(), InternalError> {
        todo!();
    }

    async fn insert_batch(
        &self,
        user_id: Id,
        feed_update_records: Vec<FeedUpdateRecord>,
    ) -> Result<(), InternalError> {
        todo!();
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        last_feed_update_record_id: Id,
    ) -> Result<Vec<FeedUpdateRecord>, InternalError> {
        todo!();
    }
}
