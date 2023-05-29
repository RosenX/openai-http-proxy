use crate::{
    service::execute_bulk_insert, InsertSqlProvider, TableDeleteOp, TableName, TablePullOp,
    TablePushOp,
};
use abi::{timestamp_to_datetime, DbService, FeedGroup, Id, InternalError, SqlValue};
use async_trait::async_trait;
use sqlx::types::chrono::Utc;

impl TableName for FeedGroup {
    fn table_name() -> String {
        "feed_group".to_string()
    }
}

impl InsertSqlProvider for FeedGroup {
    fn sql_columns() -> String {
        "user_id, name, description, update_time, sync_time, is_deleted, last_sync_device"
            .to_string()
    }
    fn sql_values(&self, user_id: Id, client_name: String) -> Vec<SqlValue> {
        vec![
            SqlValue::I32(user_id),
            SqlValue::String(self.name.clone()),
            SqlValue::NullableString(self.description.clone()),
            SqlValue::Datetime(timestamp_to_datetime(self.update_time)),
            SqlValue::Datetime(Utc::now()),
            SqlValue::Boolean(self.is_deleted),
            SqlValue::String(client_name),
        ]
    }
    fn sql_conflict() -> String {
        format!(
            "
            ON CONFLICT (user_id, name) DO UPDATE SET
                description = EXCLUDED.description,
                update_time = EXCLUDED.update_time,
                sync_time = EXCLUDED.sync_time,
                is_deleted = EXCLUDED.is_deleted,
                last_sync_device = EXCLUDED.last_sync_device
            WHERE EXCLUDED.update_time > {table_name}.update_time;
        ",
            table_name = Self::table_name()
        )
    }
}

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
                let sql = format!(
                    "SELECT * FROM feed_group WHERE user_id = {} AND sync_time > '{}' AND  last_sync_device != '{}'",
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
