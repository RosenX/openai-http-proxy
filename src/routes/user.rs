use crate::routes::authorization::{JsonWebTokenTool, JwtToken, Token};
use crate::utils::crypto::hash_password;

use crate::utils::errors::InternalError;
use crate::utils::prelude::ErrorResponse;
use crate::utils::responder::{SuccessResponse};
use crate::entities::{prelude::*, user_profile};
use rocket::fairing::AdHoc;
use rocket::serde::{Deserialize};
use rocket::serde::json::{Json};
use rocket::{post, State, routes};
use chrono::{Local};
use sea_orm::*;
use bcrypt::verify;

use super::authorization::PublicData;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SignUpInfo {
    username: String,
    email: String,
    password: String
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SignInInfo {
    email: String,
    password: String
}

impl TryFrom<SignUpInfo> for user_profile::ActiveModel {
    type Error = InternalError;
    fn try_from(info: SignUpInfo) -> Result<Self, Self::Error> {
        let now_datetime = Local::now().naive_local();

        let hashed_password = hash_password(info.password)
            .map_err(|err| InternalError::PasswordHashError(err.to_string()))?;

        let res = Self {
            username: ActiveValue::Set(info.username.clone()),
            email: ActiveValue::Set(info.email),
            hash_password: ActiveValue::Set(hashed_password),
            created_time: ActiveValue::Set(now_datetime),
            ..Default::default()
        };
        Ok(res)
    }
}

#[post("/create", data = "<info>")]
async fn register_by_email(
    info: Json<SignUpInfo>, 
    db: &State<DatabaseConnection>,
    jwt: &State<JsonWebTokenTool>) 
    ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = info.into_inner();
    let user: user_profile::ActiveModel = info.try_into()?;

    let user_model = user.insert(db.inner())
        .await
        .map_err(|err| InternalError::DuplicateEmail(err.to_string()))?;

    let tokens = jwt.encode_tokens(PublicData::from(user_model))
        .map_err(|err| InternalError::JsonWebTokenError(err.to_string()))?; 
    Ok(SuccessResponse::Created(Json(tokens)))
}

#[post("/", data = "<info>")]
async fn login_by_email(
    info: Json<SignInInfo>, 
    db: &State<DatabaseConnection>,
    jwt: &State<JsonWebTokenTool>
) ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = info.into_inner();

    let res = UserProfile::find()
        .filter(user_profile::Column::Email.eq(info.email))
        .one(db.inner())
        .await
        .map_err(|err| InternalError::DatabaseError(err.to_string()))?;
    
    match res {
        Some(user) => {
            match verify(&info.password, &user.hash_password) {
                Ok(true) => {
                    let token = jwt.encode_tokens(PublicData::from(user))
                        .map_err(|err| InternalError::JsonWebTokenError(err.to_string()))?; 
                    Ok(SuccessResponse::Accepted(Json(token)))
                },
                _ => Err(InternalError::WrongPassword.into()),
            }
        },
        None => Err(InternalError::UserNotExist.into()),
    }
}

#[post("/refresh-token", data = "<refresh_token>", format = "json")]
fn refresh_token(
    refresh_token: Json<Token>,
    jwt: &State<JsonWebTokenTool>
) ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let data =  jwt
        .decode_refresh_token(refresh_token.into_inner())
        .map_err(|err| InternalError::JsonWebTokenError(err.to_string()))?;

    let new_token = jwt
        .encode_tokens(data.data)
        .map_err(|err| InternalError::JsonWebTokenError(err.to_string()))?;
    Ok(SuccessResponse::Created(new_token.into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Stage", |rocket| async {
        rocket.mount("/user", routes![
            register_by_email, 
            login_by_email, 
            refresh_token
        ])
    })
}
