use abi::{
    AuthResponse, DbService, DecodeJwt, EncodeJwt, InternalError, JwtConfig, LoginRequest,
    PasswordVerify, RefreshTokenRequest, RegisterRequest, UserInformation, UserProfile,
};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use log::info;
use serde::Deserialize;

use crate::common::AppState;

use super::{
    user_manager::{UserManager, UserManagerOp},
    AuthService, AuthServiceApi, AuthorizedUser,
};

#[derive(Deserialize, Clone, Debug)]
pub struct AuthServiceConfig {
    pub jwt: JwtConfig,
}

impl AuthService {
    pub fn new(db_service: DbService, auth_config: AuthServiceConfig) -> Self {
        AuthService {
            config: auth_config,
            user_manager: UserManager::new(db_service),
        }
    }

    fn authurize(&self, token: abi::Token) -> Result<AuthorizedUser, InternalError> {
        let profile = token.decode_access_token(&self.config.jwt)?;
        Ok(AuthorizedUser {
            user_profile: profile,
        })
    }
}

#[async_trait]
impl AuthServiceApi for AuthService {
    type Error = InternalError;

    async fn register_by_email(
        &self,
        request: RegisterRequest,
    ) -> Result<AuthResponse, Self::Error> {
        // check request
        let register_info = request.register_info;
        let user_info = UserInformation::try_from(register_info)?;
        let user_info = self.user_manager.create(user_info).await?;
        let user_profile = UserProfile::from(user_info);
        let tokens = user_profile.encode_tokens(&self.config.jwt)?;
        Ok(AuthResponse {
            jwt_tokens: tokens,
            client: request.client,
        })
    }

    async fn login_by_email(&self, request: LoginRequest) -> Result<AuthResponse, Self::Error> {
        // check request
        let login_info = request.login_info;
        let user_info = self
            .user_manager
            .find_user_by_email(&login_info.email)
            .await?;
        let mut user_id = -1;
        let token = match user_info {
            Some(user) => match login_info.verify(&user.password) {
                Ok(true) => {
                    user_id = user.id;
                    let tokens = UserProfile::from(user).encode_tokens(&self.config.jwt)?;
                    info!("{}", tokens);
                    Ok(tokens)
                }
                _ => Err(InternalError::WrongPassword),
            },
            None => Err(InternalError::UserNotExist),
        }?;
        let client = request.client;
        let client = match client.client_id {
            Some(_) => client,
            None => {
                self.user_manager
                    .register_client(user_id, client.client_name)
                    .await?
            }
        };
        Ok(AuthResponse {
            jwt_tokens: token,
            client,
        })
    }

    fn refresh_token(&self, request: RefreshTokenRequest) -> Result<AuthResponse, Self::Error> {
        let refresh_token = request.refresh_token;
        let user_profile = refresh_token.decode_refresh_token(&self.config.jwt)?;
        let tokens = user_profile.encode_tokens(&self.config.jwt)?;
        info!("Refresh {}", tokens);
        Ok(AuthResponse {
            jwt_tokens: tokens,
            client: request.client,
        })
    }
}

// fn check_auth_header(auth_header: Option<&str>) -> Result<String, InternalError> {
//     if let Some(auth_string) = auth_header {
//         let vec_header: Vec<&str> = auth_string.split_whitespace().collect();
//         if vec_header.len() == 2 && vec_header[0] == "Bearer" {
//             return Ok(vec_header[1].to_string());
//         }
//     }
//     Err(InternalError::InvalidAuthToken("Token错误".to_string()))
// }

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
                .map_err(|e| InternalError::InvalidAuthToken(e.to_string()))?;

        match service.auth_service.authurize(bearer.token().to_string()) {
            Ok(user) => Ok(user),
            Err(err) => Err(InternalError::InvalidAuthToken(err.to_string())),
        }
    }
}
