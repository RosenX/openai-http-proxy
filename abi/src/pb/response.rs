#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tokens {
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProfile {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
    #[prost(enumeration = "ProLevel", tag = "4")]
    pub pro_level: i32,
    #[prost(int64, tag = "5")]
    pub pro_end_time: i64,
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedProfile {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub icon: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub logo: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub category_algo: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub tags_algo: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserFeed {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, tag = "2")]
    pub user_id: i32,
    #[prost(int32, tag = "3")]
    pub feed_id: i32,
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub logo: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub folder: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub tags: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, tag = "9")]
    pub created_time: i64,
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserContent {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, tag = "2")]
    pub user_id: i32,
    #[prost(int32, tag = "3")]
    pub content_id: i32,
    #[prost(enumeration = "ReadStage", tag = "4")]
    pub stage: i32,
    #[prost(string, optional, tag = "5")]
    pub tags: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub notes: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, optional, tag = "2")]
    pub feed_id: ::core::option::Option<i32>,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "4")]
    pub cover: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "5")]
    pub publish_time: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "6")]
    pub authors: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub link: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub summary_algo: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub category_algo: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub tags_algo: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, tag = "13")]
    pub create_time: i64,
    #[prost(string, tag = "14")]
    pub md5: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedContent {
    #[prost(message, optional, tag = "1")]
    pub feed_profile: ::core::option::Option<FeedProfile>,
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<Content>,
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResponse {
    #[prost(message, optional, tag = "1")]
    pub tokens: ::core::option::Option<Tokens>,
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedResponse {
    #[prost(message, optional, tag = "1")]
    pub user_feed: ::core::option::Option<UserFeed>,
    #[prost(message, optional, tag = "2")]
    pub feed_profile: ::core::option::Option<FeedProfile>,
    #[prost(message, repeated, tag = "3")]
    pub user_content: ::prost::alloc::vec::Vec<UserContent>,
    #[prost(message, repeated, tag = "4")]
    pub content: ::prost::alloc::vec::Vec<Content>,
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FecthFeedResponse {
    #[prost(message, repeated, tag = "1")]
    pub user_feeds: ::prost::alloc::vec::Vec<UserFeed>,
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchContentResponse {
    #[prost(message, repeated, tag = "1")]
    pub user_contents: ::prost::alloc::vec::Vec<UserContent>,
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<Content>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProLevel {
    Free = 0,
    Pro = 1,
}
impl ProLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProLevel::Free => "PRO_LEVEL_FREE",
            ProLevel::Pro => "PRO_LEVEL_PRO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRO_LEVEL_FREE" => Some(Self::Free),
            "PRO_LEVEL_PRO" => Some(Self::Pro),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReadStage {
    Explore = 0,
    Focus = 1,
    Seen = 2,
    Archived = 3,
}
impl ReadStage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReadStage::Explore => "READ_STAGE_EXPLORE",
            ReadStage::Focus => "READ_STAGE_FOCUS",
            ReadStage::Seen => "READ_STAGE_SEEN",
            ReadStage::Archived => "READ_STAGE_ARCHIVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "READ_STAGE_EXPLORE" => Some(Self::Explore),
            "READ_STAGE_FOCUS" => Some(Self::Focus),
            "READ_STAGE_SEEN" => Some(Self::Seen),
            "READ_STAGE_ARCHIVED" => Some(Self::Archived),
            _ => None,
        }
    }
}
