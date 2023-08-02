use abi::{timestamp_to_datetime, DbService, FeedItem, Id, InternalError, SqlValue, UserId};
use async_trait::async_trait;
use sqlx::types::chrono::Utc;

use crate::{
    service::execute_bulk_insert, InsertSqlProvider, TableDeleteOp, TableName, TablePullOp,
    TablePushOp,
};

impl TableName for FeedItem {
    fn table_name() -> String {
        "feed_item".to_string()
    }
}

impl InsertSqlProvider for FeedItem {
    fn sql_columns() -> String {
        "user_id,
        feed_url,
        is_focus,
        is_seen,
        title,
        cover,
        link,
        publish_time,
        authors,
        tags,
        category,
        description,
        summary_algo,
        create_time,
        md5_string,
        update_time,
        sync_time,
        is_deleted,
        focus_time,
        last_sync_device,
        is_marked,
        is_achieved
        "
        .to_string()
    }
    fn sql_values(&self, user_id: &UserId, client_name: String) -> Vec<SqlValue> {
        vec![
            SqlValue::String(user_id.to_owned()),
            SqlValue::String(self.feed_url.clone()),
            SqlValue::Boolean(self.is_focus),
            SqlValue::Boolean(self.is_seen),
            SqlValue::NullableString(self.title.clone()),
            SqlValue::NullableString(self.cover.clone()),
            SqlValue::NullableString(self.link.clone()),
            SqlValue::NullableDatetime(self.publish_time.map(timestamp_to_datetime)),
            SqlValue::NullableString(self.authors.clone()),
            SqlValue::NullableStringArray(self.tags.clone()),
            SqlValue::NullableString(self.category.clone()),
            SqlValue::NullableString(self.description.clone()),
            SqlValue::NullableString(self.summary_algo.clone()),
            SqlValue::Datetime(timestamp_to_datetime(self.create_time)),
            SqlValue::String(self.md5_string.clone()),
            SqlValue::Datetime(timestamp_to_datetime(self.update_time)),
            SqlValue::Datetime(Utc::now()),
            SqlValue::Boolean(self.is_deleted),
            SqlValue::NullableDatetime(self.focus_time.map(timestamp_to_datetime)),
            SqlValue::String(client_name),
            SqlValue::NullableBoolean(self.is_marked),
            SqlValue::NullableBoolean(self.is_achieved),
        ]
    }
    fn sql_conflict() -> String {
        format!(
            "
            ON CONFLICT (user_id, md5_string) DO UPDATE SET
                is_focus = EXCLUDED.is_focus,
                is_seen = EXCLUDED.is_seen,
                title = EXCLUDED.title,
                cover = EXCLUDED.cover,
                link = EXCLUDED.link,
                publish_time = EXCLUDED.publish_time,
                authors = EXCLUDED.authors,
                tags = EXCLUDED.tags,
                category = EXCLUDED.category,
                description = EXCLUDED.description,
                summary_algo = EXCLUDED.summary_algo,
                update_time = EXCLUDED.update_time,
                is_deleted = EXCLUDED.is_deleted,
                sync_time = EXCLUDED.sync_time,
                focus_time = EXCLUDED.focus_time,
                last_sync_device = EXCLUDED.last_sync_device,
                is_marked = EXCLUDED.is_marked,
                is_achieved = EXCLUDED.is_achieved
            WHERE EXCLUDED.update_time > {table_name}.update_time;
        ",
            table_name = Self::table_name()
        )
    }
}

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
        user_id: &UserId,
        last_sync_timestamp: Option<i64>,
        client_name: &str,
    ) -> Result<Vec<FeedItem>, Self::Error> {
        let result = match last_sync_timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_item WHERE user_id = '{}' AND sync_time > '{}' AND last_sync_device != '{}'",
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
        user_id: &UserId,
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
    async fn delete(db: DbService, user_id: &UserId) -> Result<(), Self::Error> {
        let sql = format!("DELETE FROM feed_item WHERE user_id = '{}'", user_id);
        sqlx::query(&sql)
            .execute(db.as_ref())
            .await
            .map_err(|e| InternalError::DatabaseDeleteError(e.to_string()))?;
        Ok(())
    }
}
