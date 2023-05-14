use chrono::Utc;
use utoipa::ToSchema;

use crate::{timestamp_to_datetime, DbTableName, Id, InsertSqlProvider, SqlValue};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ProLevel {
    Normal = 0,
    Pro = 1,
    Spro = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub client_name: String,
    pub client_id: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub pro_level: ProLevel,
    pub pro_end_time: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JwtTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedGroup {
    pub name: String,
    pub description: Option<String>,
    pub update_time: i64,
    pub is_deleted: bool,
}

impl DbTableName for FeedGroup {
    fn table_name() -> String {
        "feed_group".to_string()
    }
}

impl InsertSqlProvider for FeedGroup {
    fn sql_columns() -> String {
        "user_id, name, description, update_time, sync_time, sync_devices, is_deleted".to_string()
    }
    fn sql_values(&self, user_id: Id, client_id: Id) -> Vec<SqlValue> {
        vec![
            SqlValue::I32(user_id),
            SqlValue::String(self.name.clone()),
            SqlValue::NullableString(self.description.clone()),
            SqlValue::Datetime(timestamp_to_datetime(self.update_time)),
            SqlValue::Datetime(Utc::now()),
            SqlValue::I32Array(vec![client_id]),
            SqlValue::Boolean(self.is_deleted),
        ]
    }
    fn sql_conflict(client_id: Id) -> String {
        format!(
            "
            ON CONFLICT (user_id, name) DO UPDATE SET
                description = EXCLUDED.description,
                update_time = EXCLUDED.update_time,
                sync_time = EXCLUDED.sync_time,
                is_deleted = EXCLUDED.is_deleted,
                sync_devices = (
                    CASE
                        WHEN NOT ({client_id} = ANY({table_name}.sync_devices))
                        THEN array_append({table_name}.sync_devices, {client_id})
                        ELSE {table_name}.sync_devices
                    END
                )
            WHERE EXCLUDED.update_time > {table_name}.update_time;
        ",
            client_id = client_id,
            table_name = Self::table_name()
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedItem {
    pub feed_url: String,
    pub is_focus: bool,
    pub is_seen: bool,
    pub title: Option<String>,
    pub cover: Option<String>,
    pub link: Option<String>,
    pub publish_time: Option<i64>,
    pub authors: Option<String>,
    pub tags: Option<Vec<String>>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub summary_algo: Option<String>,
    pub create_time: i64,
    pub md5_string: String,
    pub update_time: i64,
    pub is_deleted: bool,
    pub focus_time: Option<i64>,
}

impl DbTableName for FeedItem {
    fn table_name() -> String {
        "feed_item".to_string()
    }
}

impl InsertSqlProvider for FeedItem {
    fn sql_columns() -> String {
        "user_id, feed_url, is_focus, is_seen, title, cover, link, publish_time, authors, tags, category, description, summary_algo, create_time, md5_string, update_time, sync_time, sync_devices, is_deleted, focus_time".to_string()
    }
    fn sql_values(&self, user_id: Id, client_id: Id) -> Vec<SqlValue> {
        vec![
            SqlValue::I32(user_id),
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
            SqlValue::I32Array(vec![client_id]),
            SqlValue::Boolean(self.is_deleted),
            SqlValue::NullableDatetime(self.focus_time.map(timestamp_to_datetime)),
        ]
    }
    fn sql_conflict(client_id: Id) -> String {
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
                sync_devices = (
                    CASE
                        WHEN NOT ({client_id} = ANY({table_name}.sync_devices))
                        THEN array_append({table_name}.sync_devices, {client_id})
                        ELSE {table_name}.sync_devices
                    END
                )
            WHERE EXCLUDED.update_time > {table_name}.update_time;
        ",
            client_id = client_id,
            table_name = Self::table_name()
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedUpdateRecord {
    pub feed_url: String,
    pub last_update: i64,
    pub last_content_hash: String,
    pub last_item_publish_time: Option<i64>,
    pub update_time: i64,
}

impl DbTableName for FeedUpdateRecord {
    fn table_name() -> String {
        "feed_update_record".to_string()
    }
}

// impl SqlProvider for FeedUpdateRecord
impl InsertSqlProvider for FeedUpdateRecord {
    fn sql_columns() -> String {
        "user_id, feed_url, last_update, last_content_hash, last_item_publish_time, update_time, sync_time, sync_devices"
            .to_string()
    }
    fn sql_values(&self, user_id: Id, client_id: Id) -> Vec<SqlValue> {
        vec![
            SqlValue::I32(user_id),
            SqlValue::String(self.feed_url.clone()),
            SqlValue::Datetime(timestamp_to_datetime(self.last_update)),
            SqlValue::String(self.last_content_hash.clone()),
            SqlValue::NullableDatetime(self.last_item_publish_time.map(timestamp_to_datetime)),
            SqlValue::Datetime(timestamp_to_datetime(self.update_time)),
            SqlValue::Datetime(Utc::now()),
            SqlValue::I32Array(vec![client_id]),
        ]
    }
    fn sql_conflict(client_id: Id) -> String {
        format!(
            "
            ON CONFLICT (user_id, feed_url) DO UPDATE SET
                last_update = EXCLUDED.last_update,
                last_content_hash = EXCLUDED.last_content_hash,
                last_item_publish_time = EXCLUDED.last_item_publish_time,
                update_time = EXCLUDED.update_time,
                sync_time = EXCLUDED.sync_time,
                sync_devices = (
                    CASE
                        WHEN NOT ({client_id} = ANY({table_name}.sync_devices))
                        THEN array_append({table_name}.sync_devices, {client_id})
                        ELSE {table_name}.sync_devices
                    END
                )
            WHERE EXCLUDED.update_time > {table_name}.update_time;
        ",
            client_id = client_id,
            table_name = Self::table_name()
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, sqlx::Type, ToSchema, Copy)]
#[sqlx(type_name = "feed_type", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum FeedTypeServer {
    Rss,
    Atom,
    Unknown,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub url: String,
    pub name: Option<String>,
    pub custom_name: Option<String>,
    pub logo: Option<String>,
    pub custom_logo: Option<String>,
    pub description: Option<String>,
    pub custom_description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub create_time: i64,
    pub feed_type: FeedTypeServer,
    pub update_time: i64,
    pub is_deleted: bool,
}

impl DbTableName for Feed {
    fn table_name() -> String {
        "feed".to_string()
    }
}

// impl SqlProvider for Feed
impl InsertSqlProvider for Feed {
    fn sql_columns() -> String {
        "user_id, url, name, custom_name, logo, custom_logo, description, custom_description, tags, create_time, feed_type, update_time, sync_time, sync_devices, is_deleted".to_string()
    }
    fn sql_values(&self, user_id: Id, client_id: Id) -> Vec<SqlValue> {
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
            SqlValue::EnumFeedType(self.feed_type),
            SqlValue::Datetime(timestamp_to_datetime(self.update_time)),
            SqlValue::Datetime(Utc::now()),
            SqlValue::I32Array(vec![client_id]),
            SqlValue::Bool(self.is_deleted),
        ]
    }
    fn sql_conflict(client_id: Id) -> String {
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
                is_deleted = EXCLUDED.is_deleted,
                sync_time = EXCLUDED.sync_time,
                sync_devices = (
                    CASE
                        WHEN NOT ({client_id} = ANY({table_name}.sync_devices))
                        THEN array_append({table_name}.sync_devices, {client_id})
                        ELSE {table_name}.sync_devices
                    END
                )
            WHERE EXCLUDED.update_time > {table_name}.update_time;
        ",
            client_id = client_id,
            table_name = Self::table_name()
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema, Copy)]
#[serde(rename_all = "camelCase")]
pub struct SyncTimestamp {
    pub feed: Option<i64>,
    pub feed_group: Option<i64>,
    pub feed_item: Option<i64>,
    pub feed_update_record: Option<i64>,
}
