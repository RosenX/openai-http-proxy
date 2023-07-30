use abi::{AuthingIdTokenPaylaod, InternalError, UserId};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::common::AppState;

fn authurize(token: abi::Token, secret: &String) -> Result<AuthingIdTokenPaylaod, InternalError> {
    let token = decode::<AuthingIdTokenPaylaod>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| InternalError::InvalidToken(e.to_string()))?;
    Ok(token.claims)
}

#[async_trait]
impl FromRequestParts<AppState> for AuthorizedUser
where
    AppState: Send + Sync,
{
    type Rejection = InternalError;
    async fn from_request_parts(
        parts: &mut Parts,
        service: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, service)
                .await
                .map_err(|_| InternalError::TokenNotProvided("Header中Token不存在".to_string()))?;

        match authurize(bearer.token().to_string(), &service.authing.app_secret) {
            Ok(user) => Ok(AuthorizedUser { user_profile: user }),
            Err(_) => Err(InternalError::InvalidToken("Token已失效".to_string())),
        }
    }
}

pub struct AuthorizedUser {
    user_profile: AuthingIdTokenPaylaod,
}

impl AuthorizedUser {
    pub fn get_user_id(&self) -> UserId {
        self.user_profile.sub.clone()
    }
}
