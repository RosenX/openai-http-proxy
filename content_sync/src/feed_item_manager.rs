use abi::{execute_bulk_insert, timestamp_to_datetime, DbService, FeedItem, Id, InternalError};
use async_trait::async_trait;

use crate::{TableDeleteOp, TablePullOp, TablePushOp};

#[async_trait]
pub trait FeedItemManageOp {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_items: Vec<FeedItem>,
        client_name: String,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
        client_name: String,
    ) -> Result<Vec<FeedItem>, abi::InternalError>;

    async fn delete_by_user_id(&self, user_id: Id) -> Result<(), abi::InternalError>;
}

#[async_trait]
impl TablePullOp for FeedItem {
    type Error = InternalError;
    async fn pull(
        db: DbService,
        user_id: Id,
        last_sync_timestamp: Option<i64>,
        client_name: &str,
    ) -> Result<Vec<FeedItem>, Self::Error> {
        let result = match last_sync_timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_item WHERE user_id = {} AND update_time > '{}' AND last_sync_device != '{}' AND is_deleted = false",
                    user_id,
                    timestamp_to_datetime(t),
                    client_name
                );
                sqlx::query_as::<_, FeedItem>(&sql)
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
impl TablePushOp for FeedItem {
    type Error = InternalError;
    async fn push(
        feed_items: Vec<FeedItem>,
        db: DbService,
        user_id: Id,
        client_name: &str,
    ) -> Result<(), Self::Error> {
        if feed_items.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(&db, feed_items, user_id, client_name).await?;
        Ok(())
    }
}

#[async_trait]
impl TableDeleteOp for FeedItem {
    type Error = InternalError;
    async fn delete(db: DbService, user_id: Id) -> Result<(), Self::Error> {
        let sql = format!("DELETE FROM feed_item WHERE user_id = {}", user_id);
        sqlx::query(&sql)
            .execute(db.as_ref())
            .await
            .map_err(|e| InternalError::DatabaseDeleteError(e.to_string()))?;
        Ok(())
    }
}
