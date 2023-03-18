use std::fmt::Display;

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    serde::{Deserialize, Serialize},
    Config,
};

use crate::{common::errors::InternalError, models::response::user_info::BasicUserProfile};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub data: BasicUserProfile,
    exp: usize,
}

pub type Token = String;

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data: {} \n exp: {}", self.data, self.exp)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JwtToken {
    access_token: Token,
    refresh_token: Token,
}

impl Display for JwtToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Access Token: {} \n Refresh Token: {}",
            self.access_token, self.refresh_token
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JwtService {
    access_secret_key: String,
    access_expiration_hour: i64,
    refresh_secret_key: String,
    refresh_expiration_hour: i64,
}

impl JwtService {
    pub fn new() -> Self {
        Config::figment()
            .select("jsonwebtoken")
            .extract()
            .expect("jsonwebtoken配置解析失败")
    }

    fn encode(&self, data: Claims, key: &str) -> Result<Token, InternalError> {
        let token = encode(
            &Header::default(),
            &data,
            &EncodingKey::from_secret(key.as_bytes()),
        )
        .map_err(|err| InternalError::JsonWebTokenError(err.to_string()))?;
        Ok(token)
    }

    fn decode(&self, token: Token, key: &str) -> Result<Claims, InternalError> {
        let decode_token = decode::<Claims>(
            token.as_ref(),
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::default(),
        )
        .map_err(|err| InternalError::JsonWebTokenError(err.to_string()))?;
        Ok(decode_token.claims)
    }

    fn generate_token_data(&self, data: &BasicUserProfile, expiration_time: i64) -> Claims {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(expiration_time))
            .unwrap()
            .timestamp_millis();

        Claims {
            data: data.clone(),
            exp: expiration as usize,
        }
    }

    pub fn encode_access_token(&self, data: &BasicUserProfile) -> Result<Token, InternalError> {
        let data = self.generate_token_data(data, self.access_expiration_hour);
        let token = self.encode(data, self.access_secret_key.as_ref())?;
        Ok(token)
    }

    pub fn encode_refresh_token(&self, data: &BasicUserProfile) -> Result<Token, InternalError> {
        let data = self.generate_token_data(data, self.refresh_expiration_hour);
        let token = self.encode(data, self.refresh_secret_key.as_ref())?;
        Ok(token)
    }

    pub fn encode_tokens(&self, data: BasicUserProfile) -> Result<JwtToken, InternalError> {
        let access_token = self.encode_access_token(&data)?;
        let refresh_token = self.encode_refresh_token(&data)?;
        Ok(JwtToken {
            access_token,
            refresh_token,
        })
    }

    pub fn decode_access_token(&self, token: Token) -> Result<Claims, InternalError> {
        let token_data = self.decode(token, &self.access_secret_key)?;
        Ok(token_data)
    }

    pub fn decode_refresh_token(&self, token: Token) -> Result<Claims, InternalError> {
        let token_data = self.decode(token, &self.refresh_secret_key)?;
        Ok(token_data)
    }
}
