use abi::{execute_bulk_insert, timestamp_to_datetime, DbService, FeedItem, Id, InternalError};
use async_trait::async_trait;

pub struct FeedItemManager {
    db_service: DbService,
}

impl FeedItemManager {
    pub fn new(db_service: DbService) -> Self {
        Self { db_service }
    }
}

#[async_trait]
pub trait FeedItemManageOp {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_items: Vec<FeedItem>,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
    ) -> Result<Vec<FeedItem>, abi::InternalError>;
}

#[async_trait]
impl FeedItemManageOp for FeedItemManager {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_items: Vec<FeedItem>,
    ) -> Result<(), InternalError> {
        if feed_items.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(&self.db_service, "feed_item", feed_items, user_id).await?;
        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
    ) -> Result<Vec<FeedItem>, InternalError> {
        let result = match timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_item WHERE user_id = {} AND update_time > '{}'",
                    user_id,
                    timestamp_to_datetime(t)
                );
                sqlx::query_as::<_, FeedItem>(&sql)
                    .fetch_all(self.db_service.as_ref())
                    .await?
            }
            None => vec![],
        };
        Ok(result)
    }
}
