use abi::{
    DbPool, DecodeJwt, EncodeJwt, InternalError, JwtConfig, LoginReq, PasswordVerify, RegisterReq,
    Token, Tokens, UserInformation, UserProfile,
};
use async_trait::async_trait;
use log::info;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde::Deserialize;

use super::{AuthService, AuthServiceApi, AuthorizedUser, UserManager, UserManagerOp};

#[derive(Deserialize)]
pub struct AuthConfig {
    pub jwt: JwtConfig,
}

impl AuthService {
    pub fn new(pool: DbPool, auth_config: AuthConfig) -> Self {
        AuthService {
            config: auth_config,
            user_manager: UserManager::new(pool),
        }
    }
}

#[async_trait]
impl AuthServiceApi for AuthService {
    type Error = InternalError;
    fn authurize(&self, token: abi::Token) -> Result<AuthorizedUser, Self::Error> {
        let profile = token.decode_access_token(&self.config.jwt)?;
        Ok(AuthorizedUser {
            user_profile: profile,
        })
    }

    async fn register_by_email(&self, request: RegisterReq) -> Result<abi::Tokens, Self::Error> {
        let user_info = UserInformation::try_from(request)?;
        let user_info = self.user_manager.create(user_info).await?;
        let user_profile = UserProfile::from(user_info);
        let tokens = user_profile.encode_tokens(&self.config.jwt)?;
        Ok(tokens)
    }

    async fn login_by_email(&self, request: LoginReq) -> Result<abi::Tokens, Self::Error> {
        let user_info = self.user_manager.find_user_by_email(&request.email).await?;
        match user_info {
            Some(user) => match request.verify(&user.password) {
                Ok(true) => {
                    let tokens = UserProfile::from(user).encode_tokens(&self.config.jwt)?;
                    info!("{}", tokens);
                    Ok(tokens)
                }
                _ => Err(InternalError::WrongPassword),
            },
            None => Err(InternalError::UserNotExist),
        }
    }

    fn refresh_token(&self, refresh_token: Token) -> Result<Tokens, Self::Error> {
        let user_profile = refresh_token.decode_refresh_token(&self.config.jwt)?;
        let tokens = user_profile.encode_tokens(&self.config.jwt)?;
        info!("Refresh {}", tokens);
        Ok(tokens)
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
