use crate::{
    ClientInfo, DeviceInfo, Feed, FeedGroup, FeedItem, FeedUpdateRecord, SyncTimestamp,
    READBOT_FOREVER, READBOT_ONE_MONTH, READBOT_ONE_YEAR,
};
use chrono::Duration;
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
    pub fn is_forever(&self) -> bool {
        if self.purchase_detail.product_id == READBOT_FOREVER {
            return true;
        }
        false
    }

    pub fn get_pro_duration_ms(&self) -> i64 {
        if self.purchase_detail.product_id == READBOT_ONE_MONTH {
            return Duration::days(31).num_milliseconds();
        }
        if self.purchase_detail.product_id == READBOT_ONE_YEAR {
            return Duration::days(365).num_milliseconds();
        }
        if self.is_forever() {
            return Duration::days(36500).num_milliseconds();
        }
        0
    }

    pub fn get_pro_end_time(&self) -> i64 {
        self.purchase_detail.purchase_time + self.get_pro_duration_ms()
    }
}

#[derive(Serialize, Debug)]
pub struct AppStoreVerifyRequest {
    pub receipt_data: String,
    pub password: String,
    pub exclude_old_transactions: Option<bool>,
}
