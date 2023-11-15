use crate::{
    ClientInfo, DeviceInfo, Feed, FeedGroup, FeedItem, FeedUpdateRecord, SyncTimestamp,
    APP_STORE_VERIFY_URL, APP_STORE_VERIFY_URL_SANDBOX,
};
use serde::Serialize;
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

#[derive(serde::Deserialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, ToSchema, Debug)]
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

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserActivityRequest {
    pub activity_time: i64,
    pub user_id: Option<String>,
    pub device_info: DeviceInfo,
    pub feed_num: i32,
    pub keyword_num: i32,
    pub app_version: String,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseDetail {
    pub product_id: String,
    pub purchase_time: i64,
    pub verify_data: String,
    pub source: String,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseVerifyRequest {
    pub purchase_detail: PurchaseDetail,
    pub is_test: bool,
    pub platform: String,
}

impl PurchaseVerifyRequest {
    pub fn get_verify_host(&self) -> String {
        if self.is_test {
            APP_STORE_VERIFY_URL_SANDBOX.to_string()
        } else {
            APP_STORE_VERIFY_URL.to_string()
        }
    }
}

#[derive(Serialize, Debug)]
pub struct AppStoreVerifyRequest {
    pub receipt_data: String,
    pub password: String,
    pub exclude_old_transactions: Option<bool>,
}
