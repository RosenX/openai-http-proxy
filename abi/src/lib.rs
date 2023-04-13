mod error;
mod pb;
mod types;
mod utils;

pub use error::*;
pub use pb::*;
pub use types::*;
pub use utils::*;

pub type Url = String;

pub type Hour = i64;

pub type TimestampMillis = i64;

pub type Email = String;

pub type Token = String;

pub type Id = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "feed_type", rename_all = "lowercase")]
enum FeedTypePostgres {
    Rss,
    Atom,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "pro_level", rename_all = "lowercase")]
enum ProLevelPostgres {
    Normal,
    Pro,
    Spro,
}
