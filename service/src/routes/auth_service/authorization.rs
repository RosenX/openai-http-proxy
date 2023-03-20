use abi::{DecodeJwt, InternalError, JwtConfig};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde::Deserialize;

use super::{AuthService, AuthorizedUser, Authurize};

#[derive(Deserialize)]
pub struct AuthConfig {
    pub jwt: JwtConfig,
}

impl AuthService {
    pub fn new(auth_config: AuthConfig) -> Self {
        AuthService {
            config: auth_config,
        }
    }
}

impl Authurize for AuthService {
    type Error = InternalError;
    fn authurize_user(&self, token: abi::Token) -> Result<AuthorizedUser, Self::Error> {
        let profile = token.decode_access_token(&self.config.jwt)?;
        Ok(AuthorizedUser {
            user_profile: profile,
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
            Ok(token) => match auth_service.authurize_user(token) {
                Ok(user) => Outcome::Success(user),
                Err(err) => Outcome::Failure((Status::Unauthorized, err)),
            },
            Err(err) => Outcome::Failure((Status::Unauthorized, err)),
        }
    }
}
