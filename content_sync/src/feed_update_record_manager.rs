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
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
    ) -> Result<Vec<FeedUpdateRecord>, abi::InternalError>;
}

#[async_trait]
impl FeedUpdateRecordManageOp for FeedUpdateRecordManager {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_update_records: Vec<FeedUpdateRecord>,
    ) -> Result<(), InternalError> {
        if feed_update_records.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(
            &self.db_service,
            "feed_update_record",
            feed_update_records,
            user_id,
        )
        .await?;

        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
    ) -> Result<Vec<FeedUpdateRecord>, InternalError> {
        let result = match timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT feed_id, last_update, last_content_hash, last_item_publish_time FROM feed_update_record WHERE user_id = {} AND last_update > '{}'",
                    user_id, timestamp_to_datetime(t)
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
