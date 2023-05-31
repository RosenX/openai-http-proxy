use crate::{ClientInfo, Feed, FeedGroup, FeedItem, FeedUpdateRecord, SyncTimestamp};
use utoipa::ToSchema;

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedInfo {
    pub url: String,
    pub image: Option<String>,
    pub title: Option<String>,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub client: ClientInfo,
    pub login_info: LoginInfo,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModifyPasswordRequest {
    pub client: ClientInfo,
    pub login_info: LoginInfo,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub client: ClientInfo,
    pub register_info: RegisterInfo,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    pub client: ClientInfo,
    pub refresh_token: String,
}

#[derive(serde::Deserialize, ToSchema, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContentPullRequest {
    pub client: ClientInfo,
    pub sync_timestamp: SyncTimestamp,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeFeedRequest {
    pub client: ClientInfo,
    pub feed_info: FeedInfo,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentPushRequest {
    pub feeds: Vec<Feed>,
    pub feed_update_records: Vec<FeedUpdateRecord>,
    pub feed_groups: Vec<FeedGroup>,
    pub feed_items: Vec<FeedItem>,
    pub client: ClientInfo,
}
