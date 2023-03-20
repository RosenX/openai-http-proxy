use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{DecodeJwt, EncodeJwt, InternalError, JwtConfig, Token, Tokens, UserInformation};

#[derive(Serialize)]
pub struct RegisterResponse {
    tokens: Tokens,
}

impl From<Tokens> for RegisterResponse {
    fn from(tokens: Tokens) -> Self {
        Self { tokens }
    }
}

// todo，挪个位置
#[derive(Serialize, Deserialize, Clone)]
pub struct UserProfile {
    id: i32,
    email: String,
    username: String,
    pro_level: i32,
    pro_end_time: DateTime<Utc>,
}

impl From<UserInformation> for UserProfile {
    fn from(user_profile: UserInformation) -> Self {
        Self {
            id: user_profile.id,
            email: user_profile.email,
            username: user_profile.username,
            pro_level: user_profile.pro_level,
            pro_end_time: user_profile.pro_end_time,
        }
    }
}

impl EncodeJwt for UserProfile {
    type Error = InternalError;
    fn encode_tokens(self, config: &JwtConfig) -> Result<Tokens, Self::Error> {
        let tokens = Tokens {
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
