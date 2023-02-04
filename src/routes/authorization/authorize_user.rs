use chrono::{NaiveDateTime};
use rocket::{request::{FromRequest, Outcome}, Request, http::Status, serde::{Serialize, Deserialize}};

use crate::{entities::user_profile, utils::jwt::{structs::JsonWebToken, jwt::JsonWebTokenTool}};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct PublicData {
    pub user_id: i32,
    pub is_pro: bool,
    pub pro_end_time: Option<NaiveDateTime>
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

pub struct AuthorizedProUser {
    pub user_id: i32,
}

fn check_auth_header(auth_header: Option<&str>) -> Result<String, ()> {
    if let Some(auth_string)= auth_header {
        let vec_header: Vec<&str> = auth_string.split_whitespace().collect();
        if vec_header.len() == 2 && vec_header[0] == "Bearer" {
            return Ok(vec_header[1].to_string())
        }
        else {
            return Err(())
        }
    }
    Err(())
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        let jwt = match request.rocket().state::<JsonWebToken>() {
            Some(jwt) => jwt,
            None => return Outcome::Failure((Status::Unauthorized, ())),
        };

        match check_auth_header(auth_header) {
            Ok(auth_token) => match JsonWebTokenTool::decode_access_token(
                    auth_token.into(),
                    jwt
                ) {
                Ok(token) => Outcome::Success(
                    AuthorizedUser { 
                        user_id: token.claims.jwt_data.user_id
                    }
                ),
                _ => Outcome::Failure((Status::Unauthorized, ())),
            },
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}