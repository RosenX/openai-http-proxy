use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::{
    common::{errors::InternalError, responder::ErrorInfo},
    models::response::user_info::BasicUserProfile,
};

use super::jwt::JsonWebTokenTool;

pub type AuthorizedUser = BasicUserProfile;

fn check_auth_header(auth_header: Option<&str>) -> Result<String, InternalError> {
    if let Some(auth_string) = auth_header {
        let vec_header: Vec<&str> = auth_string.split_whitespace().collect();
        if vec_header.len() == 2 && vec_header[0] == "Bearer" {
            return Ok(vec_header[1].to_string());
        }
    }
    return Err(InternalError::InvalidAuthToken("Token错误".to_string()));
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = InternalError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        let jwt = request.rocket().state::<JsonWebTokenTool>().unwrap();

        let auth_token = check_auth_header(auth_header);

        match auth_token {
            Ok(token) => match jwt.decode_access_token(token.into()) {
                Ok(data) => Outcome::Success(data.data),
                Err(err) => Outcome::Failure((
                    Status::Unauthorized,
                    InternalError::JsonWebTokenError(err.to_string()),
                )),
            },
            Err(err) => Outcome::Failure((
                Status::Unauthorized,
                InternalError::TokenExpired(err.to_string()),
            )),
        }
    }
}
