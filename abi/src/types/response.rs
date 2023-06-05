use crate::{ClientInfo, Feed, FeedGroup, FeedItem, FeedUpdateRecord, JwtTokens, SyncTimestamp};
use serde::{Deserialize, Serialize};
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

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeFeedResponse {
    pub client: ClientInfo,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub status_code: u16,
    pub error_code: Option<u32>,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Default for Response<T> {
    fn default() -> Self {
        Self {
            status_code: 200,
            error_code: None,
            message: "Success".to_string(),
            data: None,
        }
    }
}

impl<T> Response<T> {
    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
    pub fn new(status_code: u16, error_code: Option<u32>, message: String) -> Self {
        Self {
            status_code,
            error_code,
            message,
            data: None,
        }
    }
}
