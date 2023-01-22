use chrono::Utc;
use rocket::serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use crate::utils::{prelude::FailureJsonResponder};

use super::{PublicData, Token};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub jwt_data: PublicData,
    exp: usize
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JwtToken {
    token: Token,
    refresh_token: Token
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JsonWebTokenConfig {
    access_secret_key: String,
    access_expiration_hour: i64,
    refresh_secret_key: String,
    refresh_expiration_hour: i64,  
}

pub struct JsonWebTokenTool{
    config: JsonWebTokenConfig,
}

impl JsonWebTokenTool {
    pub fn new(jwt_config: JsonWebTokenConfig) -> JsonWebTokenTool {
        JsonWebTokenTool{
            config: jwt_config
        }
    }

    fn encode_jwt(jwt_data: &PublicData, secret: &str, expiration_time: i64)
    -> Result<String, FailureJsonResponder<String>> 
    {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(expiration_time)).unwrap()
            .timestamp();
        
        let my_claims = Claims {
            jwt_data: jwt_data.to_owned(),
            exp: expiration as usize,
        };

        let token = encode(
            &Header::default(), 
            &my_claims, 
            &EncodingKey::from_secret(secret.as_ref())
        )?;
        Ok(token)
    }

    fn decode_jwt(token: String, secret: &str)
    -> Result<TokenData<Claims>, FailureJsonResponder<String>>
    {
        let decode_token = decode::<Claims>(
            &token, 
            &DecodingKey::from_secret(secret.as_ref()), 
            &Validation::default()
        )?;
        Ok(decode_token)
    }

    pub fn encode_access_token(jwt_data: &PublicData, config: &JsonWebTokenConfig)
        -> Result<Token, FailureJsonResponder<String>> 
    {
        let token = Self::encode_jwt(
            jwt_data,
            &config.access_secret_key,
            config.access_expiration_hour
        )?;
        Ok(Token{token})
    }

    pub fn encode_refresh_token(jwt_data: &PublicData, config: &JsonWebTokenConfig)
        -> Result<Token, FailureJsonResponder<String>> 
    {
        let token = Self::encode_jwt(
            jwt_data, 
            &config.refresh_secret_key,
            config.refresh_expiration_hour
        )?;
        Ok(Token{token})
    }

    pub fn encode_token(jwt_data: PublicData, config: &JsonWebTokenConfig)
        -> Result<JwtToken, FailureJsonResponder<String>>
    {
        let token = Self::encode_access_token(&jwt_data, config)?;
        let refresh_token = Self::encode_refresh_token(&jwt_data, config)?;
        Ok(JwtToken{token, refresh_token})
    }

    pub fn decode_access_token<'a>(token: Token, config: &'a JsonWebTokenConfig)
    -> Result<TokenData<Claims>, FailureJsonResponder<String>> 
    {
        let token_data = Self::decode_jwt(
            token.into(), 
            &config.access_secret_key
        )?;
        Ok(token_data)
    }

    pub fn decode_refresh_token<'a>(token: Token, config: &'a JsonWebTokenConfig)
        -> Result<TokenData<Claims>, FailureJsonResponder<String>> 
    {
        let token_data = Self::decode_jwt(
            token.into(), 
            &config.refresh_secret_key
        )?;
        Ok(token_data)
    }

}

