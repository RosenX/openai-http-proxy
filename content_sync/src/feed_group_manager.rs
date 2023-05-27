use crate::{TableDeleteOp, TablePullOp, TablePushOp};
use abi::{execute_bulk_insert, timestamp_to_datetime, DbService, FeedGroup, Id, InternalError};
use async_trait::async_trait;

#[async_trait]
impl TablePullOp for FeedGroup {
    type Error = InternalError;
    async fn pull(
        db: DbService,
        user_id: Id,
        last_sync_timestamp: Option<i64>,
        client_name: &str,
    ) -> Result<Vec<FeedGroup>, Self::Error> {
        let result = match last_sync_timestamp {
            Some(t) => {
                // TODO: update time change to sync time
                let sql = format!(
                    "SELECT * FROM feed_group WHERE user_id = {} AND update_time > '{}' AND  last_sync_device != '{}' AND is_deleted = false",
                    user_id,
                    timestamp_to_datetime(t),
                    client_name
                );
                sqlx::query_as::<_, FeedGroup>(&sql)
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
impl TablePushOp for FeedGroup {
    type Error = InternalError;
    async fn push(
        feed_groups: Vec<FeedGroup>,
        db: DbService,
        user_id: Id,
        client_name: &str,
    ) -> Result<(), Self::Error> {
        if feed_groups.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(&db, feed_groups, user_id, client_name).await?;
        Ok(())
    }
}

#[async_trait]
impl TableDeleteOp for FeedGroup {
    type Error = InternalError;
    async fn delete(db: DbService, user_id: Id) -> Result<(), Self::Error> {
        // TODO feed_group name
        let sql = format!("DELETE FROM feed_group WHERE user_id = {}", user_id);
        sqlx::query(&sql)
            .execute(db.as_ref())
            .await
            .map_err(|e| InternalError::DatabaseDeleteError(e.to_string()))?;
        Ok(())
    }
}
