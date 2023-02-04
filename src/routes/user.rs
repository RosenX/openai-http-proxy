use log::{info, warn, debug, error};
use crate::database::DbOperator;
use crate::utils::crypto::hash_password;

use crate::utils::jwt::jwt::JsonWebTokenTool;
use crate::utils::jwt::structs::{JwtToken, JsonWebToken, Token};
use crate::utils::jwt::traits::Encode;
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
    type Error = anyhow::Error;
    fn try_from(info: SignUpInfo) -> Result<Self, Self::Error> {
        let now_datetime = Local::now().naive_local();
        let hashed_password = hash_password(info.password)
            .map_err(|err| {
                error!("{}", err.to_string());
                err
            })?;
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
    jwt: &State<JsonWebToken>) 
    ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = info.into_inner();
    let user: user_profile::ActiveModel = info.try_into()?;
    let user_model = db.inner().insert_item(user).await?;
    let tokens = jwt.encode(user_model.into())?;
    Ok(SuccessResponse::Created(Json(tokens)))
}

#[post("/", data = "<info>")]
async fn login_by_email(
    info: Json<SignInInfo>, 
    db: &State<DatabaseConnection>,
    jwt: &State<JsonWebToken>
) ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = info.into_inner();

    let res = UserProfile::find()
        .filter(user_profile::Column::Email.eq(info.email))
        .one(db.inner())
        .await
        .map_err(|_| ErrorResponse::user_not_exist())?;
    
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
                        .map_err(|_| ErrorResponse::jwt_encode_fail())?;
                    Ok(SuccessResponse::Accepted(Json(token)))
                },
                _ => return Err(ErrorResponse::invalid_password()),
            }
        },
        None => Err(ErrorResponse::user_not_exist())
    }
}

#[post("/refresh-token", data = "<refresh_token>", format = "json")]
fn refresh_token(
    refresh_token: Json<Token>,
    jwt: &State<JsonWebToken>
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
                .map_err(|_| ErrorResponse::jwt_encode_fail())?;
            Ok(SuccessResponse::Created(new_token.into()))
        }
        Err(_) => Err(ErrorResponse::invalid_refresh_token())
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
