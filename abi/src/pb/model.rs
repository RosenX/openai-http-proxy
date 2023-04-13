#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tokens {
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedGroup {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub update_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItem {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bool, tag = "2")]
    pub is_focus: bool,
    #[prost(bool, tag = "3")]
    pub is_seen: bool,
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub link: ::prost::alloc::string::String,
    #[prost(int64, tag = "9")]
    pub publish_time: i64,
    #[prost(string, tag = "10")]
    pub authors: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "11")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "12")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub summary_algo: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub content: ::prost::alloc::string::String,
    #[prost(bool, tag = "16")]
    pub content_have_parsed: bool,
    #[prost(int64, tag = "17")]
    pub create_time: i64,
    #[prost(string, tag = "18")]
    pub md5_string: ::prost::alloc::string::String,
    #[prost(int32, tag = "19")]
    pub feed_id: i32,
    #[prost(int64, tag = "20")]
    pub update_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedUpdateRecord {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, tag = "2")]
    pub feed_id: i32,
    #[prost(int64, tag = "3")]
    pub last_update: i64,
    #[prost(string, tag = "4")]
    pub last_content_hash: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub last_item_publish_time: i64,
    #[prost(int64, tag = "6")]
    pub update_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub custom_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub logo: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub custom_logo: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub custom_description: ::prost::alloc::string::String,
    #[prost(int32, tag = "9")]
    pub group_id: i32,
    #[prost(string, repeated, tag = "10")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "11")]
    pub create_time: i64,
    #[prost(enumeration = "FeedType", tag = "12")]
    pub feed_type: i32,
    #[prost(int64, tag = "13")]
    pub update_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncTimestamp {
    #[prost(int64, tag = "1")]
    pub feed: i64,
    #[prost(int64, tag = "2")]
    pub feed_group: i64,
    #[prost(int64, tag = "3")]
    pub feed_item: i64,
    #[prost(int64, tag = "4")]
    pub feed_update_record: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProLevel {
    Normal = 0,
    Pro = 1,
    Spro = 2,
}
impl ProLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProLevel::Normal => "PRO_LEVEL_NORMAL",
            ProLevel::Pro => "PRO_LEVEL_PRO",
            ProLevel::Spro => "PRO_LEVEL_SPRO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRO_LEVEL_NORMAL" => Some(Self::Normal),
            "PRO_LEVEL_PRO" => Some(Self::Pro),
            "PRO_LEVEL_SPRO" => Some(Self::Spro),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FeedType {
    Rss = 0,
    Atom = 1,
    Unknown = 2,
}
impl FeedType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FeedType::Rss => "RSS",
            FeedType::Atom => "ATOM",
            FeedType::Unknown => "Unknown",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RSS" => Some(Self::Rss),
            "ATOM" => Some(Self::Atom),
            "Unknown" => Some(Self::Unknown),
            _ => None,
        }
    }
}
