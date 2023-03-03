use crate::common::responder::{SuccessResponse, ErrorResponse};
use crate::common::utils::crypto::PasswordVerify;
use crate::models::request::login_req::LoginReq;
use crate::models::request::register_req::RegisterReq;
use crate::routes::authorization::{JsonWebTokenTool, JwtToken, Token};

use crate::common::errors::InternalError;
use crate::entities::{prelude::*, user_profile};
use rocket::fairing::AdHoc;
use rocket::serde::json::{Json};
use rocket::{post, State, routes};
use sea_orm::*;

use super::authorization::PublicData;

#[post("/register", data = "<info>")]
async fn register_by_email(
    info: Json<RegisterReq>, 
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

#[post("/login", data = "<info>")]
async fn login_by_email(
    info: Json<LoginReq>, 
    db: &State<DatabaseConnection>,
    jwt: &State<JsonWebTokenTool>
) ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = info.into_inner();

    match info.find_user_by_email(db.inner()).await? {
        Some(user) => {
            match info.verify(&user.hash_password) {
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
