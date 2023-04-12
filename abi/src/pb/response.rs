#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResponse {
    #[prost(message, optional, tag = "1")]
    pub tokens: ::core::option::Option<super::model::Tokens>,
}
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentPullResponse {
    #[prost(message, repeated, tag = "1")]
    pub feeds: ::prost::alloc::vec::Vec<super::model::Feed>,
    #[prost(message, repeated, tag = "2")]
    pub feed_update_records: ::prost::alloc::vec::Vec<super::model::FeedUpdateRecord>,
    #[prost(message, repeated, tag = "3")]
    pub feed_groups: ::prost::alloc::vec::Vec<super::model::FeedGroup>,
    #[prost(message, repeated, tag = "4")]
    pub feed_items: ::prost::alloc::vec::Vec<super::model::FeedItem>,
}
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentPushResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
