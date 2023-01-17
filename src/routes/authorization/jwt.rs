use std::str::FromStr;

use chrono::Utc;
use rocket::serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use crate::utils::{prelude::FailureJsonResponder};
use crate::utils::crypto::{SECRET, REFRESH_SECRET, EXPIRATION_TIME, REFRESH_EXPIRATION_TIME};

use super::JwtData;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub jwt_data: JwtData,
    exp: usize
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    token: String,
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Token{token: s}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct JwtToken {
    pub token: Token,
    pub refresh_token: Token
}


fn encode_jwt(jwt_data: JwtData, secret: &str, expiration_time: i64) 
    -> Result<String, FailureJsonResponder<String>> 
{
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(expiration_time)).unwrap()
        .timestamp();
    
    let my_claims = Claims {
        jwt_data: jwt_data,
        exp: expiration as usize,
    };

    let token = encode(
        &Header::default(), 
        &my_claims, 
        &EncodingKey::from_secret(secret.as_ref())
    )?;
    Ok(token)
}

fn decode_jwt(token: String, secret: &'static str) 
    -> Result<TokenData<Claims>, FailureJsonResponder<String>>
{
    let decode_token = decode::<Claims>(
        &token, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    )?;
    Ok(decode_token)
}

pub fn encode_token(jwt_data: JwtData) 
    -> Result<JwtToken, FailureJsonResponder<String>> 
{
    let token = encode_jwt(jwt_data.clone(), SECRET, EXPIRATION_TIME)?;
    let refresh_token = encode_jwt(jwt_data.clone(), REFRESH_SECRET, REFRESH_EXPIRATION_TIME)?;
    Ok(JwtToken{token: token.into(), refresh_token: refresh_token.into()})
}

pub fn decode_access_token(token: String) 
    -> Result<TokenData<Claims>, FailureJsonResponder<String>> 
{
    let token = decode_jwt(token, SECRET)?;
    Ok(token)
}

pub fn decode_refresh_token(token: Token) 
    -> Result<TokenData<Claims>, FailureJsonResponder<String>> 
{
    let token = decode_jwt(token.token, REFRESH_SECRET)?;
    Ok(token)
}