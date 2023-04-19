use utoipa::ToSchema;

use crate::{Id, InsertSqlProvider, SqlValue};

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
}

impl InsertSqlProvider for FeedGroup {
    fn sql_columns() -> String {
        "user_id, name, description, update_time".to_string()
    }
    fn sql_values(&self, user_id: Id) -> Vec<SqlValue> {
        vec![
            SqlValue::I32(user_id),
            SqlValue::String(self.name.clone()),
            SqlValue::NullableString(self.description.clone()),
            SqlValue::I64(self.update_time),
        ]
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
}

impl InsertSqlProvider for FeedItem {
    fn sql_columns() -> String {
        "user_id, feed_url, is_focus, is_seen, title, cover, link, publish_time, authors, tags, category, description, summary_algo, create_time, md5_string, update_time".to_string()
    }
    fn sql_values(&self, user_id: Id) -> Vec<SqlValue> {
        vec![
            SqlValue::I32(user_id),
            SqlValue::String(self.feed_url.clone()),
            SqlValue::Boolean(self.is_focus),
            SqlValue::Boolean(self.is_seen),
            SqlValue::NullableString(self.title.clone()),
            SqlValue::NullableString(self.cover.clone()),
            SqlValue::NullableString(self.link.clone()),
            SqlValue::NullableI64(self.publish_time),
            SqlValue::NullableString(self.authors.clone()),
            SqlValue::NullableStringArray(self.tags.clone()),
            SqlValue::NullableString(self.category.clone()),
            SqlValue::NullableString(self.description.clone()),
            SqlValue::NullableString(self.summary_algo.clone()),
            SqlValue::I64(self.create_time),
            SqlValue::String(self.md5_string.clone()),
            SqlValue::I64(self.update_time),
        ]
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

// impl SqlProvider for FeedUpdateRecord
impl InsertSqlProvider for FeedUpdateRecord {
    fn sql_columns() -> String {
        "user_id, feed_url, last_update, last_content_hash, last_item_publish_time, update_time"
            .to_string()
    }
    fn sql_values(&self, user_id: Id) -> Vec<SqlValue> {
        vec![
            SqlValue::I32(user_id),
            SqlValue::String(self.feed_url.clone()),
            SqlValue::I64(self.last_update),
            SqlValue::String(self.last_content_hash.clone()),
            SqlValue::NullableI64(self.last_item_publish_time),
            SqlValue::I64(self.update_time),
        ]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, sqlx::Type, ToSchema)]
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
}

// impl SqlProvider for Feed
impl InsertSqlProvider for Feed {
    fn sql_columns() -> String {
        "user_id, url, name, custom_name, logo, custom_logo, description, custom_description, tags, create_time, feed_type, update_time".to_string()
    }
    fn sql_values(&self, user_id: Id) -> Vec<SqlValue> {
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
            SqlValue::I64(self.create_time),
            SqlValue::String(self.feed_type.to_string()),
            SqlValue::I64(self.update_time),
        ]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SyncTimestamp {
    pub feed: Option<i64>,
    pub feed_group: Option<i64>,
    pub feed_item: Option<i64>,
    pub feed_update_record: Option<i64>,
}
