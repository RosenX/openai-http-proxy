use crate::{
    AuthResponse, DecodeJwt, EncodeJwt, InternalError, JwtConfig, Token, Tokens, UserInformation,
    UserProfile,
};

impl From<UserInformation> for UserProfile {
    fn from(user_info: UserInformation) -> Self {
        Self {
            id: user_info.id,
            email: user_info.email,
            username: user_info.username,
            pro_level: user_info.pro_level as i32,
            pro_end_time: user_info.pro_end_time.timestamp(),
        }
    }
}

impl From<Tokens> for AuthResponse {
    fn from(value: Tokens) -> Self {
        Self {
            tokens: Some(value),
        }
    }
}

// impl Display for UserProfile {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "user_id: {}, email: {}, username: {}, pro_level: {}, pro_end_time: {}",
//             self.id, self.email, self.username, self.pro_level, self.pro_end_time
//         )
//     }
// }

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
