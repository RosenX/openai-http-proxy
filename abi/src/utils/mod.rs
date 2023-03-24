mod crypto;
mod http;
mod jwt;
mod postgres;

use chrono::{DateTime, NaiveDateTime, Utc};
pub use crypto::{EncryptUtil, PasswordEncrypt, PasswordVerify};
pub use http::HttpService;
pub use jwt::{DecodeJwt, EncodeJwt, JwtConfig, Payload};
pub use postgres::{DatabaseConfig, DbOption, DbService};

// convert timestamp to Datetime<utc>
pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0);
    // if naive is None, return Utc::now()
    match naive {
        Some(naive) => DateTime::<Utc>::from_utc(naive, Utc),
        None => Utc::now(),
    }
}

// convert Datetime<utc> to timestamp
pub fn datetime_to_timestamp(datetime: DateTime<Utc>) -> i64 {
    datetime.timestamp()
}
