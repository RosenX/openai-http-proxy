#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterInfo {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginInfo {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshToken {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfo {
    #[prost(string, tag = "1")]
    pub client_name: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(message, optional, tag = "1")]
    pub client_info: ::core::option::Option<ClientInfo>,
    #[prost(message, optional, tag = "2")]
    pub login_info: ::core::option::Option<LoginInfo>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(message, optional, tag = "1")]
    pub client_info: ::core::option::Option<ClientInfo>,
    #[prost(message, optional, tag = "2")]
    pub register_info: ::core::option::Option<RegisterInfo>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshTokenRequest {
    #[prost(message, optional, tag = "1")]
    pub client_info: ::core::option::Option<ClientInfo>,
    #[prost(message, optional, tag = "2")]
    pub refresh_token: ::core::option::Option<RefreshToken>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentPullRequest {
    #[prost(message, optional, tag = "1")]
    pub sync_timestamp: ::core::option::Option<super::model::SyncTimestamp>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentPushRequest {
    #[prost(message, repeated, tag = "1")]
    pub feeds: ::prost::alloc::vec::Vec<super::model::Feed>,
    #[prost(message, repeated, tag = "2")]
    pub feed_update_records: ::prost::alloc::vec::Vec<super::model::FeedUpdateRecord>,
    #[prost(message, repeated, tag = "3")]
    pub feed_groups: ::prost::alloc::vec::Vec<super::model::FeedGroup>,
    #[prost(message, repeated, tag = "4")]
    pub feed_items: ::prost::alloc::vec::Vec<super::model::FeedItem>,
}
