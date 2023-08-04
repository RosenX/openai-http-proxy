use abi::{timestamp_to_datetime, DbService, FeedUpdateRecord, InternalError, SqlValue, UserId};
use async_trait::async_trait;
use sqlx::types::chrono::Utc;

use crate::{
    service::execute_bulk_insert, InsertSqlProvider, TableDeleteOp, TableName, TablePullOp,
    TablePushOp,
};

impl TableName for FeedUpdateRecord {
    fn table_name() -> String {
        "feed_update_record".to_string()
    }
}

// impl SqlProvider for FeedUpdateRecord
impl InsertSqlProvider for FeedUpdateRecord {
    fn sql_columns() -> String {
        "user_id, feed_url, last_update, last_content_hash, last_item_publish_time, update_time, sync_time, last_sync_device, is_deleted, failed_count"
            .to_string()
    }
    fn sql_values(&self, user_id: &UserId, client_name: String) -> Vec<SqlValue> {
        vec![
            SqlValue::String(user_id.to_owned()),
            SqlValue::String(self.feed_url.clone()),
            SqlValue::Datetime(timestamp_to_datetime(self.last_update)),
            SqlValue::String(self.last_content_hash.clone()),
            SqlValue::NullableDatetime(self.last_item_publish_time.map(timestamp_to_datetime)),
            SqlValue::Datetime(timestamp_to_datetime(self.update_time)),
            SqlValue::Datetime(Utc::now()),
            SqlValue::String(client_name),
            SqlValue::Boolean(self.is_deleted),
            SqlValue::NullableInt(self.failed_count),
        ]
    }
    fn sql_conflict() -> String {
        format!(
            "
            ON CONFLICT (user_id, feed_url) DO UPDATE SET
                last_update = EXCLUDED.last_update,
                last_content_hash = EXCLUDED.last_content_hash,
                last_item_publish_time = EXCLUDED.last_item_publish_time,
                update_time = EXCLUDED.update_time,
                sync_time = EXCLUDED.sync_time,
                is_deleted = EXCLUDED.is_deleted,
                last_sync_device = EXCLUDED.last_sync_device,
                failed_count = EXCLUDED.failed_count
            WHERE EXCLUDED.update_time > {table_name}.update_time;
        ",
            table_name = Self::table_name()
        )
    }
}

#[async_trait]
impl TablePullOp for FeedUpdateRecord {
    type Error = InternalError;
    async fn pull(
        db: DbService,
        user_id: &UserId,
        last_sync_timestamp: Option<i64>,
        client_name: &str,
    ) -> Result<Vec<FeedUpdateRecord>, Self::Error> {
        let result = match last_sync_timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_update_record WHERE user_id = '{}' AND sync_time > '{}' AND last_sync_device != '{}'",
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
        user_id: &UserId,
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
    async fn delete(db: DbService, user_id: &UserId) -> Result<(), Self::Error> {
        let sql = format!(
            "DELETE FROM feed_update_record WHERE user_id = '{}'",
            user_id
        );
        sqlx::query(&sql)
            .execute(db.as_ref())
            .await
            .map_err(|e| InternalError::DatabaseDeleteError(e.to_string()))?;
        Ok(())
    }
}
