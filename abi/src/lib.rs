mod error;
mod models;
mod pb;
mod types;
mod utils;

pub use error::*;
pub use models::*;
pub use pb::*;
pub use types::*;
pub use utils::*;

const DEFAULT_ID: i32 = -1;
const UNKNOWN: &str = "unknown"; // todo，database schema change
const SEP: &str = ",";

pub type HttpClient = reqwest::Client;

pub type Url = String;

pub type Hour = i64;

pub type TimestampMillis = i64;

pub type Email = String;

pub type Token = String;

pub type Id = i32;
