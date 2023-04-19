use crate::{ClientInfo, Feed, FeedGroup, FeedItem, FeedUpdateRecord, JwtTokens, SyncTimestamp};
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub jwt_tokens: JwtTokens,
    pub client: ClientInfo,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentPullResponse {
    pub sync_timestamp: SyncTimestamp,
    pub feeds: Vec<Feed>,
    pub feed_update_records: Vec<FeedUpdateRecord>,
    pub feed_groups: Vec<FeedGroup>,
    pub feed_items: Vec<FeedItem>,
    pub client: ClientInfo,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentPushResponse {
    pub client: ClientInfo,
    pub message: String,
}
