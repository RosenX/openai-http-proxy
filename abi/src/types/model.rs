use serde::Deserialize;
use utoipa::ToSchema;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ProLevel {
    Normal = 0,
    Pro = 1,
    Spro = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub client_name: String,
    pub client_id: Option<i32>, // TODO: remove this field
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
    pub sync_time: Option<i64>,
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
    pub sync_time: Option<i64>,
    pub is_marked: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedUpdateRecord {
    pub feed_url: String,
    pub last_update: i64,
    pub last_content_hash: String,
    pub last_item_publish_time: Option<i64>,
    pub update_time: i64,
    pub is_deleted: bool,
    pub sync_time: Option<i64>,
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
    pub feed_type: Option<FeedTypeServer>,
    pub update_time: i64,
    pub is_deleted: bool,
    pub sync_time: Option<i64>,
    pub group_name: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, ToSchema, Copy, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SyncTimestamp {
    pub feed: Option<i64>,
    pub feed_group: Option<i64>,
    pub feed_item: Option<i64>,
    pub feed_update_record: Option<i64>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(unused)]
pub struct AuthingIdTokenPaylaod {
    pub sub: String,
    aud: String,
    iat: i64,
    exp: i64,
    iss: String,
    name: Option<String>,
    given_name: Option<String>,
    middle_name: Option<String>,
    family_name: Option<String>,
    nickname: Option<String>,
    preferred_username: Option<String>,
    profile: Option<String>,
    picture: Option<String>,
    website: Option<String>,
    birthdate: Option<String>,
    gender: Option<String>,
    zoneinfo: Option<String>,
    locale: Option<String>,
    updated_at: Option<String>,
    email: Option<String>,
    email_verified: Option<bool>,
    phone_number: Option<String>,
    phone_number_verified: Option<bool>,
    address: Option<Address>,
    username: Option<String>,
    role: Option<Vec<String>>,
    extended_fields: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Address {
    country: Option<String>,
    postal_code: Option<String>,
    region: Option<String>,
    formatted: Option<String>,
}
