use abi::{execute_bulk_insert, timestamp_to_datetime, DbService, Feed, Id, InternalError};
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
    async fn insert_batch(
        &self,
        user_id: Id,
        feeds: Vec<Feed>,
        client_name: String,
    ) -> Result<(), abi::InternalError>;

    async fn insert(
        &self,
        user_id: Id,
        feeds: Feed,
        client_name: String,
    ) -> Result<(), abi::InternalError>;

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
        client_name: String,
    ) -> Result<Vec<Feed>, abi::InternalError>;

    async fn delete_by_user_id(&self, user_id: Id) -> Result<(), abi::InternalError>;
}

#[async_trait]
impl FeedManageOp for FeedManager {
    async fn insert_batch(
        &self,
        user_id: Id,
        feeds: Vec<Feed>,
        client_name: String,
    ) -> Result<(), InternalError> {
        if feeds.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(&self.db_service, feeds, user_id, client_name).await?;
        Ok(())
    }

    async fn insert(
        &self,
        user_id: Id,
        feeds: Feed,
        client_name: String,
    ) -> Result<(), InternalError> {
        let feeds = vec![feeds];
        execute_bulk_insert(&self.db_service, feeds, user_id, client_name).await?;
        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
        client_name: String,
    ) -> Result<Vec<Feed>, InternalError> {
        let result = match timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed WHERE user_id = {} AND update_time > '{}' AND last_sync_device != '{}' AND is_deleted = false",
                    user_id,
                    timestamp_to_datetime(t),
                    client_name
                );
                sqlx::query_as::<_, Feed>(&sql)
                    .fetch_all(self.db_service.as_ref())
                    .await
                    .map_err(|e| InternalError::DatabaseSelectError(e.to_string()))?
            }
            None => vec![],
        };
        Ok(result)
    }

    async fn delete_by_user_id(&self, user_id: Id) -> Result<(), InternalError> {
        let sql = format!("DELETE FROM feed WHERE user_id = {}", user_id);
        sqlx::query(&sql)
            .execute(self.db_service.as_ref())
            .await
            .map_err(|e| InternalError::DatabaseDeleteError(e.to_string()))?;
        Ok(())
    }
}
