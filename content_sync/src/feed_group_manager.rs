use abi::{execute_bulk_insert, timestamp_to_datetime, DbService, FeedGroup, Id, InternalError};
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
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_groups: Vec<FeedGroup>,
        client_id: Id,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
        client_id: Id,
    ) -> Result<Vec<FeedGroup>, abi::InternalError>;

    async fn delete_by_user_id(&self, user_id: Id) -> Result<(), abi::InternalError>;
}

#[async_trait]
impl FeedGroupManageOp for FeedGroupManager {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_groups: Vec<FeedGroup>,
        client_id: Id,
    ) -> Result<(), InternalError> {
        if feed_groups.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(&self.db_service, feed_groups, user_id, client_id).await?;
        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
        client_id: Id,
    ) -> Result<Vec<FeedGroup>, InternalError> {
        let result = match timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_group WHERE user_id = {} AND update_time > '{}' AND NOT ({} = ANY (sync_devices))",
                    user_id,
                    timestamp_to_datetime(t),
                    client_id
                );
                sqlx::query_as::<_, FeedGroup>(&sql)
                    .fetch_all(self.db_service.as_ref())
                    .await
                    .map_err(|e| InternalError::DatabaseSelectError(e.to_string()))?
            }
            None => vec![],
        };
        Ok(result)
    }

    async fn delete_by_user_id(&self, user_id: Id) -> Result<(), InternalError> {
        let sql = format!("DELETE FROM feed_group WHERE user_id = {}", user_id);
        sqlx::query(&sql)
            .execute(self.db_service.as_ref())
            .await
            .map_err(|e| InternalError::DatabaseDeleteError(e.to_string()))?;
        Ok(())
    }
}
