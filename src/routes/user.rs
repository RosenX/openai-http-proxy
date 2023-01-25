use crate::routes::authorization::{Token};
use crate::utils::crypto::hash_password;

use crate::utils::prelude::ErrorResponse;
use crate::utils::responder::{SuccessResponse};
use crate::entities::{prelude::*, user_profile};
use rocket::fairing::AdHoc;
use rocket::serde::{Deserialize};
use rocket::serde::json::{Json};
use rocket::{post, State, routes};
use chrono::{Local};
use super::authorization::{PublicData, JwtToken, JsonWebTokenTool, JsonWebTokenConfig};
use sea_orm::*;
use bcrypt::verify;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RegisterInfo {
    username: String,
    email: String,
    password: String
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoginInfo {
    email: String,
    password: String
}

#[post("/create", data = "<info>")]
async fn register_by_email(
    info: Json<RegisterInfo>, 
    db: &State<DatabaseConnection>,
    jwt: &State<JsonWebTokenConfig>) 
    ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = info.into_inner();

    let now_datetime = Local::now().naive_local();
    println!("{}", now_datetime);

    let hashed_password = 
        hash_password(info.password).map_err(|_| ErrorResponse::HashError())?;

    let user = user_profile::ActiveModel {
        username: ActiveValue::Set(info.username.clone()),
        email: ActiveValue::Set(info.email),
        hash_password: ActiveValue::Set(hashed_password),
        created_time: ActiveValue::Set(now_datetime),
        ..Default::default()
    };
    
    let user = 
        user.insert(db.inner()).await.map_err(|_| ErrorResponse::InvalidEmail())?;

    let tokens = JsonWebTokenTool::encode_token(PublicData{
        user_id: user.id,
        is_pro: user.is_pro,
        pro_end_time: user.pro_end_time
    }, jwt.inner()).map_err(|_| ErrorResponse::JwtEncodeFail())?;

    Ok(SuccessResponse::Created(Json(tokens)))
}

#[post("/", data = "<info>")]
async fn login_by_email(
    info: Json<LoginInfo>, 
    db: &State<DatabaseConnection>,
    jwt: &State<JsonWebTokenConfig>
) ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = info.into_inner();

    let res = UserProfile::find()
        .filter(user_profile::Column::Email.eq(info.email))
        .one(db.inner())
        .await
        .map_err(|_| ErrorResponse::UserNotExist())?;
    
    match res {
        Some(user) => {
            match verify(&info.password, &user.hash_password) {
                Ok(true) => {
                    let token = 
                        JsonWebTokenTool::encode_token(
                            PublicData {
                                user_id: user.id, 
                                is_pro: user.is_pro, 
                                pro_end_time: user.pro_end_time 
                            }, 
                            jwt.inner()
                        )
                        .map_err(|_| ErrorResponse::JwtEncodeFail())?;
                    Ok(SuccessResponse::Accepted(Json(token)))
                },
                _ => return Err(ErrorResponse::InvalidPassword()),
            }
        },
        None => Err(ErrorResponse::UserNotExist())
    }
}

#[post("/refresh-token", data = "<refresh_token>", format = "json")]
async fn refresh_token(
    refresh_token: Json<Token>,
    jwt: &State<JsonWebTokenConfig>
) ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    println!("{:?}", refresh_token.clone().into_inner());
    let result =  
        JsonWebTokenTool::decode_refresh_token(
            refresh_token.into_inner(),
            jwt.inner()
        );
    match result {
        Ok(token_data) => {
            let new_token = 
                JsonWebTokenTool::encode_token(
                    token_data.claims.jwt_data,
                    jwt.inner()
                )
                .map_err(|_| ErrorResponse::JwtEncodeFail())?;
            Ok(SuccessResponse::Created(new_token.into()))
        }
        Err(_) => Err(ErrorResponse::InvalidRefreshToken())
    }
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
