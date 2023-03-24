mod content;
mod user;
mod user_content;

pub use content::*;
pub use user::*;
pub use user_content::*;

use crate::{DecodeJwt, EncodeJwt, InternalError, JwtConfig, Token, Tokens, UserProfile};

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
