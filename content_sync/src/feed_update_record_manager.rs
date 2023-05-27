use abi::{
    execute_bulk_insert, timestamp_to_datetime, DbService, FeedUpdateRecord, Id, InternalError,
};
use async_trait::async_trait;

use crate::{TableDeleteOp, TablePullOp, TablePushOp};

#[async_trait]
impl TablePullOp for FeedUpdateRecord {
    type Error = InternalError;
    async fn pull(
        db: DbService,
        user_id: Id,
        last_sync_timestamp: Option<i64>,
        client_name: &str,
    ) -> Result<Vec<FeedUpdateRecord>, Self::Error> {
        let result = match last_sync_timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_update_record WHERE user_id = {} AND update_time > '{}' AND last_sync_device != '{}'",
                    user_id,
                    timestamp_to_datetime(t),
                    client_name
                );
                sqlx::query_as::<_, FeedUpdateRecord>(&sql)
                    .fetch_all(db.as_ref())
                    .await
                    .map_err(|e| InternalError::DatabaseSelectError(e.to_string()))?
            }
            None => vec![],
        };
        Ok(result)
    }
}

#[async_trait]
impl TablePushOp for FeedUpdateRecord {
    type Error = InternalError;
    async fn push(
        feed_update_records: Vec<FeedUpdateRecord>,
        db: DbService,
        user_id: Id,
        client_name: &str,
    ) -> Result<(), Self::Error> {
        if feed_update_records.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(&db, feed_update_records, user_id, client_name).await?;
        Ok(())
    }
}

#[async_trait]
impl TableDeleteOp for FeedUpdateRecord {
    type Error = InternalError;
    async fn delete(db: DbService, user_id: Id) -> Result<(), Self::Error> {
        let sql = format!("DELETE FROM feed_update_record WHERE user_id = {}", user_id);
        sqlx::query(&sql)
            .execute(db.as_ref())
            .await
            .map_err(|e| InternalError::DatabaseDeleteError(e.to_string()))?;
        Ok(())
    }
}
