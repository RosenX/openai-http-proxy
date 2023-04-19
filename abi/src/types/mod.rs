mod content;
mod model;
mod request;
mod response;
mod user;

pub use model::*;
pub use request::*;
pub use response::*;
pub use user::*;

use crate::{DecodeJwt, EncodeJwt, InternalError, JwtConfig, Token};

impl EncodeJwt for UserProfile {
    type Error = InternalError;
    fn encode_tokens(self, config: &JwtConfig) -> Result<JwtTokens, Self::Error> {
        let tokens = JwtTokens {
            access_token: self
                .clone()
                .encode_token(&config.access_key, config.access_expiration_hour)?,
            refresh_token: self
                .encode_token(&config.refresh_key, config.refresh_expiration_hour)?,
        };
        Ok(tokens)
    }
}

impl DecodeJwt<UserProfile> for Token {
    type Error = InternalError;
    fn decode_access_token(self, config: &JwtConfig) -> Result<UserProfile, Self::Error> {
        let payload = self.decode(&config.access_key)?;
        Ok(payload.data)
    }
    fn decode_refresh_token(self, config: &JwtConfig) -> Result<UserProfile, Self::Error> {
        let payload = self.decode(&config.refresh_key)?;
        Ok(payload.data)
    }
}

pub trait ToSql {
    fn to_insert_sql(&self, values: Vec<Self>) -> String
    where
        Self: Sized;
}

pub trait OptionDisplay {
    fn display(self) -> String;
}

impl OptionDisplay for Option<String> {
    fn display(self) -> String {
        match self {
            Some(string) => string,
            None => "null".to_string(),
        }
    }
}

impl OptionDisplay for Option<i64> {
    fn display(self) -> String {
        match self {
            Some(i64) => i64.to_string(),
            None => "null".to_string(),
        }
    }
}

impl OptionDisplay for Option<bool> {
    fn display(self) -> String {
        match self {
            Some(bool) => bool.to_string(),
            None => "null".to_string(),
        }
    }
}

impl OptionDisplay for Option<Vec<String>> {
    fn display(self) -> String {
        match self {
            // to posgres array
            Some(vec) => format!("{{{}}}", vec.join(",")),
            None => "null".to_string(),
        }
    }
}
