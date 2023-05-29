use abi::{timestamp_to_datetime, DbService, Feed, Id, InternalError, SqlValue};
use async_trait::async_trait;
use sqlx::types::chrono::Utc;

use crate::{
    service::execute_bulk_insert, InsertSqlProvider, TableDeleteOp, TableName, TablePullOp,
    TablePushOp,
};

impl TableName for Feed {
    fn table_name() -> String {
        "feed".to_string()
    }
}

// impl SqlProvider for Feed
impl InsertSqlProvider for Feed {
    fn sql_columns() -> String {
        "user_id,
        url,
        name,
        custom_name,
        logo,
        custom_logo,
        description,
        custom_description,
        tags,
        create_time,
        feed_type,
        update_time,
        sync_time,
        is_deleted,
        last_sync_device
        "
        .to_string()
    }
    fn sql_values(&self, user_id: Id, client_name: String) -> Vec<SqlValue> {
        vec![
            SqlValue::I32(user_id),
            SqlValue::String(self.url.clone()),
            SqlValue::NullableString(self.name.clone()),
            SqlValue::NullableString(self.custom_name.clone()),
            SqlValue::NullableString(self.logo.clone()),
            SqlValue::NullableString(self.custom_logo.clone()),
            SqlValue::NullableString(self.description.clone()),
            SqlValue::NullableString(self.custom_description.clone()),
            SqlValue::NullableStringArray(self.tags.clone()),
            SqlValue::Datetime(timestamp_to_datetime(self.create_time)),
            SqlValue::NullableEnumFeedType(self.feed_type),
            SqlValue::Datetime(timestamp_to_datetime(self.update_time)),
            SqlValue::Datetime(Utc::now()),
            SqlValue::Bool(self.is_deleted),
            SqlValue::String(client_name),
        ]
    }
    fn sql_conflict() -> String {
        format!(
            "
            ON CONFLICT (user_id, url) DO UPDATE SET
                name = EXCLUDED.name,
                custom_name = EXCLUDED.custom_name,
                logo = EXCLUDED.logo,
                custom_logo = EXCLUDED.custom_logo,
                description = EXCLUDED.description,
                custom_description = EXCLUDED.custom_description,
                tags = EXCLUDED.tags,
                update_time = EXCLUDED.update_time,
                feed_type = EXCLUDED.feed_type,
                is_deleted = EXCLUDED.is_deleted,
                sync_time = EXCLUDED.sync_time,
                last_sync_device = EXCLUDED.last_sync_device
            WHERE EXCLUDED.update_time > {table_name}.update_time;
        ",
            table_name = Self::table_name()
        )
    }
}

#[async_trait]
impl TablePullOp for Feed {
    type Error = InternalError;
    async fn pull(
        db: DbService,
        user_id: Id,
        last_sync_timestamp: Option<i64>,
        client_name: &str,
    ) -> Result<Vec<Feed>, Self::Error> {
        let result = match last_sync_timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed WHERE user_id = {} AND sync_time > '{}' AND last_sync_device != '{}'",
                    user_id,
                    timestamp_to_datetime(t),
                    client_name
                );
                sqlx::query_as::<_, Feed>(&sql)
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
impl TablePushOp for Feed {
    type Error = InternalError;
    async fn push(
        feeds: Vec<Feed>,
        db: DbService,
        user_id: Id,
        client_name: &str,
    ) -> Result<(), Self::Error> {
        if feeds.is_empty() {
            return Ok(());
        }
        execute_bulk_insert(&db, feeds, user_id, client_name).await?;
        Ok(())
    }
}

#[async_trait]
impl TableDeleteOp for Feed {
    type Error = InternalError;
    async fn delete(db: DbService, user_id: Id) -> Result<(), Self::Error> {
        let sql = format!("DELETE FROM feed WHERE user_id = {}", user_id);
        sqlx::query(&sql)
            .execute(db.as_ref())
            .await
            .map_err(|e| InternalError::DatabaseDeleteError(e.to_string()))?;
        Ok(())
    }
}
