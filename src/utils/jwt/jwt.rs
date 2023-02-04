use chrono::Utc;
use log::error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use anyhow::{Result, Ok};
use rocket::serde::{Serialize, Deserialize};

use super::{traits::{Encode, Decode}, structs::{JwtToken, Token}};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims<T> {
    pub data: T,
    exp: usize
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JsonWebTokenTool {
    access_secret_key: String,
    access_expiration_hour: i64,
    refresh_secret_key: String,
    refresh_expiration_hour: i64,  
}

impl<T: Serialize> Encode<Claims<T>, Token> for JsonWebTokenTool {
    type Error = anyhow::Error;
    fn encode(&self, data: Claims<T>, key: &str) 
        -> Result<Token, Self::Error> 
    {    
        let token = encode(
            &Header::default(), 
            &data, 
            &EncodingKey::from_secret(key.as_bytes())
        )?;
        Ok(token.into())
    }
}

impl <'a, T: Deserialize<'a>> Decode<Token, T> for JsonWebTokenTool{
    type Error = anyhow::Error;
    fn decode(&self, token: Token, key: &str) 
        -> Result<T, Self::Error> 
    {
        let decode_token = decode::<Claims<T>>(
            token.as_ref(), 
            &DecodingKey::from_secret(key.as_bytes()), 
            &Validation::default()
        )?;
        Ok(decode_token.claims.data)
    }
}

impl JsonWebTokenTool {
    fn generate_token_data<From>(&self, data: From, expiration_time: i64) 
        -> Claims<From>
    {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(expiration_time)).unwrap()
            .timestamp();
        let my_claims = Claims {
            data: data,
            exp: expiration as usize,
        };
        my_claims
    }

    pub fn encode_access_token<From:Serialize>(&self, data: From)
        -> Result<Token> 
    {
        let data = self.generate_token_data(
            data, 
            self.access_expiration_hour,
        );
        let token = self.encode(data, self.access_secret_key.as_ref())?;
        Ok(token)
    }

    pub fn encode_refresh_token<From:Serialize>(&self, data: &From)
        -> Result<Token> 
    {
        let data = self.generate_token_data(
            data, 
            self.refresh_expiration_hour,
        );
        let token = self.encode(data, self.refresh_secret_key.as_ref())?;
        Ok(token)
    }

    pub fn encode_tokens<From: Serialize>(&self, data: From) ->
        Result<JwtToken> 
    {
        let access_token = self.encode_access_token(&data)?;
        let refresh_token = self.encode_refresh_token(&data)?;
        Ok(JwtToken{access_token, refresh_token})
    }

    pub fn decode_access_token<'a, To: Deserialize<'a>>(&self, token: Token)
    -> Result<To> 
    {
        let token_data = self.decode(
            token, 
            &self.access_secret_key
        )?;
        Ok(token_data)
    }

    pub fn decode_refresh_token<'a, To: Deserialize<'a>>(&self, token: Token)
        -> Result<To>
    {
        let token_data = self.decode(
            token, 
            &self.refresh_secret_key
        )?;
        Ok(token_data)
    }

}


