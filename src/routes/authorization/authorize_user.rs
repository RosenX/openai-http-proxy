use chrono::{NaiveDateTime};
use rocket::{request::{FromRequest, Outcome}, Request, http::Status, serde::{Serialize, Deserialize}};

use crate::{entities::user_profile, utils::errors::InternalError};

use super::jwt::JsonWebTokenTool;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct PublicData {
    user_id: i32,
    is_pro: bool,
    pro_end_time: Option<NaiveDateTime>,
}

impl From<user_profile::Model> for PublicData {
    fn from(user_model: user_profile::Model) -> Self {
        Self {
            user_id: user_model.id,
            is_pro: user_model.is_pro,
            pro_end_time: user_model.pro_end_time
        }
    }
}

pub struct AuthorizedUser {
    pub user_id: i32,
}

fn check_auth_header(auth_header: Option<&str>) 
    -> Result<String, InternalError> 
{
    if let Some(auth_string)= auth_header {
        let vec_header: Vec<&str> = auth_string.split_whitespace().collect();
        if vec_header.len() == 2 && vec_header[0] == "Bearer" {
            return Ok(vec_header[1].to_string());
        }
    }
    return Err(InternalError::InvalidAuthToken(
        "auth token in http head is invalid".to_string()
    ));
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = InternalError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        let jwt = request.rocket().state::<JsonWebTokenTool>().unwrap();

        let auth_token = check_auth_header(auth_header);
        
        match auth_token {
            Ok(token) => {
                match jwt.decode_access_token(token.into()) {
                    Ok(data) => {
                        Outcome::Success(
                            AuthorizedUser {
                                user_id: data.data.user_id
                            }
                        )
                    },
                    Err(err) => {
                        Outcome::Failure((
                            Status::Unauthorized, 
                            InternalError::JsonWebTokenError(err.to_string())
                        ))
                    }
                }
            }
            Err(err) => {
                Outcome::Failure((
                    Status::Unauthorized, 
                    InternalError::JsonWebTokenError(err.to_string())
                ))
            }
        }
    }
}