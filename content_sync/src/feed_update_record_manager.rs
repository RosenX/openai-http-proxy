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
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_update_records: Vec<FeedUpdateRecord>,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: i64,
    ) -> Result<Vec<FeedUpdateRecord>, abi::InternalError>;
}

#[async_trait]
impl FeedUpdateRecordManageOp for FeedUpdateRecordManager {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_update_records: Vec<FeedUpdateRecord>,
    ) -> Result<(), InternalError> {
        let values = feed_update_records
            .iter()
            .map(|feed_update_record| {
                format!(
                    "({},{},{},{},{})",
                    user_id,
                    feed_update_record.feed_id,
                    feed_update_record.last_update,
                    feed_update_record.last_content_hash,
                    feed_update_record.last_item_publish_time
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let sql = format!( "INSERT INTO feed_update_record (user_id, feed_id, last_update, last_content_hash, last_item_publish_time) VALUES {}", values);
        sqlx::query(&sql).execute(self.db_service.as_ref()).await?;
        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: i64,
    ) -> Result<Vec<FeedUpdateRecord>, InternalError> {
        let sql = format!(
            "SELECT feed_id, last_update, last_content_hash, last_item_publish_time FROM feed_update_record WHERE user_id = {} AND last_update > {}",
            user_id, timestamp
        );
        let feed_update_records = sqlx::query_as::<_, FeedUpdateRecord>(&sql)
            .fetch_all(self.db_service.as_ref())
            .await?;
        Ok(feed_update_records)
    }
}
