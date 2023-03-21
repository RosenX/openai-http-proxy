use std::fmt::Display;

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{Hour, TimestampMillis};

pub type Token = String;

#[derive(Serialize)]
pub struct Tokens {
    pub access_token: Token,
    pub refresh_token: Token,
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "access token: {}, refresh token: {}",
            self.access_token, self.refresh_token
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Payload<T> {
    pub data: T,
    exp: TimestampMillis,
}

#[derive(Deserialize)]
pub struct JwtConfig {
    pub access_key: String,
    pub access_expiration_hour: i64,
    pub refresh_key: String,
    pub refresh_expiration_hour: i64,
}

pub trait EncodeJwt {
    type Error;
    fn generate_payload(self, expiration_time: Hour) -> Payload<Self>
    where
        Self: Sized,
    {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(expiration_time))
            .unwrap()
            .timestamp_millis();
        Payload {
            data: self,
            exp: expiration,
        }
    }

    fn encode_token(
        self,
        key: &str,
        expiration_time: Hour,
    ) -> Result<Token, jsonwebtoken::errors::Error>
    where
        Self: Sized + Serialize + Clone,
    {
        let payload = self.clone().generate_payload(expiration_time);
        let token = self.encode(payload, key)?;
        Ok(token)
    }

    fn encode(self, data: Payload<Self>, key: &str) -> Result<Token, jsonwebtoken::errors::Error>
    where
        Self: Sized + Serialize,
    {
        let token = encode(
            &Header::default(),
            &data,
            &EncodingKey::from_secret(key.as_bytes()),
        )?;
        Ok(token)
    }

    fn encode_tokens(self, config: &JwtConfig) -> Result<Tokens, Self::Error>;
}

pub trait DecodeJwt<T> {
    type Error;
    // todo: 这个生命周期是什么意思
    fn decode(self, key: &str) -> Result<Payload<T>, jsonwebtoken::errors::Error>
    where
        T: Sized + for<'a> Deserialize<'a>,
        Self: Sized + AsRef<str>,
    {
        let decode_token = decode::<Payload<T>>(
            self.as_ref(),
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::default(),
        )?;
        Ok(decode_token.claims)
    }

    fn decode_access_token(self, config: &JwtConfig) -> Result<T, Self::Error>
    where
        Self: Sized + for<'a> Deserialize<'a>;

    fn decode_refresh_token(self, config: &JwtConfig) -> Result<T, Self::Error>
    where
        Self: Sized + for<'a> Deserialize<'a>;
}

// pub trait Jwt<T> {
//     type Error;
//     fn generate_access_payload(&self, data: T) -> AccessPayload<T>;
//     fn generate_refresh_payload(&self) -> RefreshPayload;
//     fn encode_access_token(&self, data: T) -> Result<Token, Self::Error>;
//     fn encode_refresh_token(&self) -> Result<Token, Self::Error>;
//     fn decode_access_token(&self, token: Token) -> Result<T, Self::Error>;
//     fn decode_refresh_token(&self, token: Token) -> Result<(), InternalError>;
//     fn encode_tokens(&self, data: T) -> Result<Tokens, InternalError>;
// }

// impl Jwt<BasicUserProfile> for JwtTool {
//     type Error = InternalError;
//     fn generate_access_payload(&self, data: BasicUserProfile) -> AccessPayload<BasicUserProfile> {
//         let expiration = Utc::now()
//             .checked_add_signed(chrono::Duration::hours(self.access_expiration_hour))
//             .unwrap()
//             .timestamp_millis();
//         AccessPayload {
//             data: data,
//             exp: expiration,
//         }
//     }

//     fn generate_refresh_payload(&self) -> RefreshPayload {
//         let expiration = Utc::now()
//             .checked_add_signed(chrono::Duration::hours(self.refresh_expiration_hour))
//             .unwrap()
//             .timestamp_millis();
//         RefreshPayload { exp: expiration }
//     }

//     fn encode_access_token(&self, data: BasicUserProfile) -> Result<Token, Self::Error> {
//         let payload = self.generate_access_payload(data);
//         let token = encode(
//             &Header::default(),
//             &payload,
//             &EncodingKey::from_secret(self.access_key.as_bytes()),
//         )?;
//         Ok(token)
//     }

//     fn encode_refresh_token(&self) -> Result<Token, Self::Error> {
//         let payload = self.generate_refresh_payload();
//         let token = encode(
//             &Header::default(),
//             &payload,
//             &EncodingKey::from_secret(self.access_key.as_bytes()),
//         )?;
//         Ok(token)
//     }

//     fn decode_access_token(&self, token: Token) -> Result<BasicUserProfile, Self::Error> {
//         let decode_token = decode::<AccessPayload<BasicUserProfile>>(
//             token.as_ref(),
//             &DecodingKey::from_secret(self.access_key.as_bytes()),
//             &Validation::default(),
//         )?;
//         Ok(decode_token.claims.data)
//     }

//     fn decode_refresh_token(&self, token: Token) -> Result<(), InternalError> {
//         let _ = decode::<RefreshPayload>(
//             token.as_ref(),
//             &DecodingKey::from_secret(self.refresh_key.as_bytes()),
//             &Validation::default(),
//         )?;
//         Ok(())
//     }

//     fn encode_tokens(&self, data: BasicUserProfile) -> Result<Tokens, InternalError> {
//         let tokens = Tokens {
//             access_token: self.encode_access_token(data)?,
//             refresh_token: self.encode_refresh_token()?,
//         };
//         Ok(tokens)
//     }
// }
