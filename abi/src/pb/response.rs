#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResponse {
    #[prost(message, optional, tag = "1")]
    pub jwt_tokens: ::core::option::Option<super::model::JwtTokens>,
    #[prost(message, optional, tag = "2")]
    pub client: ::core::option::Option<super::model::ClientInfo>,
}
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentPullResponse {
    #[prost(message, optional, tag = "1")]
    pub sync_timestamp: ::core::option::Option<super::model::SyncTimestamp>,
    #[prost(message, repeated, tag = "2")]
    pub feeds: ::prost::alloc::vec::Vec<super::model::Feed>,
    #[prost(message, repeated, tag = "3")]
    pub feed_update_records: ::prost::alloc::vec::Vec<super::model::FeedUpdateRecord>,
    #[prost(message, repeated, tag = "4")]
    pub feed_groups: ::prost::alloc::vec::Vec<super::model::FeedGroup>,
    #[prost(message, repeated, tag = "5")]
    pub feed_items: ::prost::alloc::vec::Vec<super::model::FeedItem>,
    #[prost(message, optional, tag = "6")]
    pub client: ::core::option::Option<super::model::ClientInfo>,
}
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentPushResponse {
    #[prost(message, optional, tag = "1")]
    pub client: ::core::option::Option<super::model::ClientInfo>,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
