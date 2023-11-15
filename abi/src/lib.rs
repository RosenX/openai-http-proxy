#![deny(unused_crate_dependencies)]

mod error;
mod types;
mod utils;

pub use error::*;
pub use types::*;
pub use utils::*;

pub type Url = String;

pub type Hour = i64;

pub type TimestampMillis = i64;

pub type Email = String;

pub type Token = String;

pub type Id = i32;

pub type UserId = String;

pub const INSERT_CHUNK_SIZE: usize = 50;

pub const APP_STORE_VERIFY_URL: &str = "https://buy.itunes.apple.com/verifyReceipt";
pub const APP_STORE_VERIFY_URL_SANDBOX: &str = "https://sandbox.itunes.apple.com/verifyReceipt";

pub const PLATFORM_APPLE: &str = "apple";

pub const READBOT_FOREVER: &str = "readbot_forever";
pub const READBOT_ONE_MONTH: &str = "readbot_1_month";
pub const READBOT_ONE_YEAR: &str = "readbot_1_year";

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "pro_level", rename_all = "lowercase")]
enum ProLevelPostgres {
    Normal,
    Pro,
    Spro,
}
