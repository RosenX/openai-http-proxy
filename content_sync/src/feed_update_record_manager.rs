use abi::{
    execute_bulk_insert, timestamp_to_datetime, DbService, FeedUpdateRecord, Id, InternalError,
};
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
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_update_records: Vec<FeedUpdateRecord>,
        client_id: Id,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
        client_id: Id,
    ) -> Result<Vec<FeedUpdateRecord>, abi::InternalError>;
}

#[async_trait]
impl FeedUpdateRecordManageOp for FeedUpdateRecordManager {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_update_records: Vec<FeedUpdateRecord>,
        client_id: Id,
    ) -> Result<(), InternalError> {
        if feed_update_records.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(&self.db_service, feed_update_records, user_id, client_id).await?;

        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
        client_id: Id,
    ) -> Result<Vec<FeedUpdateRecord>, InternalError> {
        let result = match timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_group WHERE user_id = {} AND update_time > '{}' AND NOT ({} = ANY(sync_devices))",
                    user_id,
                    timestamp_to_datetime(t),
                    client_id
                );
                sqlx::query_as::<_, FeedUpdateRecord>(&sql)
                    .fetch_all(self.db_service.as_ref())
                    .await?
            }
            None => vec![],
        };
        Ok(result)
    }
}
