mod error;
mod models;
mod utils;

pub use error::*;
pub use models::*;
pub use utils::*;

use sqlx::MySql;

const DEFAULT_ID: i32 = -1;
const UNKNOWN: &str = "unknown"; // todo，database schema change
const SEP: &str = ",";

pub type DbPool = sqlx::Pool<MySql>;

pub type HttpClient = reqwest::Client;

pub type Url = String;

pub type Hour = i64;

pub type TimestampMillis = i64;
