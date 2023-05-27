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
        client_name: String,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
        client_name: String,
    ) -> Result<Vec<FeedUpdateRecord>, abi::InternalError>;

    async fn delete_by_user_id(&self, user_id: Id) -> Result<(), abi::InternalError>;
}

#[async_trait]
impl FeedUpdateRecordManageOp for FeedUpdateRecordManager {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_update_records: Vec<FeedUpdateRecord>,
        client_name: String,
    ) -> Result<(), InternalError> {
        if feed_update_records.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(&self.db_service, feed_update_records, user_id, client_name).await?;
        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
        client_name: String,
    ) -> Result<Vec<FeedUpdateRecord>, InternalError> {
        let result = match timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_update_record WHERE user_id = {} AND update_time > '{}' AND last_sync_device != '{}'",
                    user_id,
                    timestamp_to_datetime(t),
                    client_name
                );
                sqlx::query_as::<_, FeedUpdateRecord>(&sql)
                    .fetch_all(self.db_service.as_ref())
                    .await
                    .map_err(|e| InternalError::DatabaseSelectError(e.to_string()))?
            }
            None => vec![],
        };
        Ok(result)
    }

    async fn delete_by_user_id(&self, user_id: Id) -> Result<(), InternalError> {
        let sql = format!("DELETE FROM feed_update_record WHERE user_id = {}", user_id);
        sqlx::query(&sql)
            .execute(self.db_service.as_ref())
            .await
            .map_err(|e| InternalError::DatabaseDeleteError(e.to_string()))?;
        Ok(())
    }
}
