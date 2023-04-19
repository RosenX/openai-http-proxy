use abi::{
    AuthResponse, DbService, DecodeJwt, EncodeJwt, InternalError, JwtConfig, LoginRequest,
    PasswordVerify, RefreshTokenRequest, RegisterRequest, UserInformation, UserProfile,
};
use async_trait::async_trait;
use log::info;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde::Deserialize;

use super::{
    user_manager::{UserManager, UserManagerOp},
    AuthService, AuthServiceApi, AuthorizedUser,
};

#[derive(Deserialize)]
pub struct AuthConfig {
    pub jwt: JwtConfig,
}

impl AuthService {
    pub fn new(db_service: DbService, auth_config: AuthConfig) -> Self {
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

fn check_auth_header(auth_header: Option<&str>) -> Result<String, InternalError> {
    if let Some(auth_string) = auth_header {
        let vec_header: Vec<&str> = auth_string.split_whitespace().collect();
        if vec_header.len() == 2 && vec_header[0] == "Bearer" {
            return Ok(vec_header[1].to_string());
        }
    }
    Err(InternalError::InvalidAuthToken("Token错误".to_string()))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = InternalError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        let access_token = check_auth_header(auth_header);

        let auth_service = request.rocket().state::<AuthService>().unwrap();

        match access_token {
            Ok(token) => match auth_service.authurize(token) {
                Ok(user) => Outcome::Success(user),
                Err(err) => Outcome::Failure((Status::Unauthorized, err)),
            },
            Err(err) => Outcome::Failure((Status::Unauthorized, err)),
        }
    }
}
