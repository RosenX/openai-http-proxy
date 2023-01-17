use crate::routes::authorization::{AuthorizedUser, Token};
use crate::utils::crypto::hash_password;

use crate::utils::responder::{SuccessJsonResponder, FailureJsonResponder, BodyData};
use crate::utils::{prelude::*, SUCCESS};
use crate::entities::{prelude::*, user_profile};
use rocket::fairing::AdHoc;
use rocket::response::Redirect;
use rocket::serde::{Deserialize};
use rocket::serde::json::{Json};
use rocket::{post, State, routes, get, uri};
use chrono::{Local};
use super::authorization::{encode_token, JwtData, decode_refresh_token, JwtToken};
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
async fn user_register(info: Json<RegisterInfo>, db: &State<DatabaseConnection>) 
    ->  Result<SuccessJsonResponder<JwtToken>, FailureJsonResponder<String>>
{
    let info = info.into_inner();

    let now_datetime = Local::now().naive_local();
    println!("{}", now_datetime);

    let hashed_password = hash_password(info.password)?;
    println!("{}", hashed_password);

    let user = user_profile::ActiveModel {
        username: ActiveValue::Set(info.username.clone()),
        email: ActiveValue::Set(info.email),
        hash_password: ActiveValue::Set(hashed_password),
        created_time: ActiveValue::Set(now_datetime),
        ..Default::default()
    };
    
    let user = user.insert(db.inner()).await?;

    let tokens = encode_token(JwtData{
        user_id: user.id,
        is_pro: user.is_pro,
        pro_end_time: user.pro_end_time
    })?;

    Ok(BodyData{data: tokens}.into())
}

#[post("/", data = "<info>")]
async fn user_login(info: Json<LoginInfo>, db: &State<DatabaseConnection>) 
    ->  Result<SuccessJsonResponder<JwtToken>, FailureJsonResponder<String>>
{
    let info = info.into_inner();

    let res = UserProfile::find()
        .filter(user_profile::Column::Email.eq(info.email))
        .one(db.inner())
        .await?;
    
    match res {
        Some(user) => {
            match verify(&info.password, &user.hash_password) {
                Ok(true) => {
                    let token = encode_token(JwtData { 
                        user_id: user.id, 
                        is_pro: user.is_pro, 
                        pro_end_time: user.pro_end_time 
                    })?;
                    Ok(BodyData{data: token}.into())
                }
                Ok(_) => Err(BodyData{data: "Wrong Password".to_string()}.into()),
                Err(_) => Err(BodyData{data: "Wrong Password".to_string()}.into()),
            }
        },
        None => return Err(BodyData{data: "User Not Exist".to_string()}.into())
    }
}

#[post("/refresh-token", data = "<refresh_token>", format = "json")]
async fn refresh_token(
    refresh_token: Json<Token>) 
    ->  Result<SuccessJsonResponder<JwtToken>, FailureJsonResponder<String>>
{
    println!("{:?}", refresh_token.clone().into_inner());
    let result =  decode_refresh_token(refresh_token.into_inner());
    match result {
        Ok(token_data) => {
            let new_token = encode_token(token_data.claims.jwt_data)?;
            Ok(BodyData{data: new_token}.into())
        }
        Err(_) => Err(BodyData{data: "invalid refresh token".to_string()}.into())
    }
}

#[get("/")]
async fn get_user_info(
    db: &State<DatabaseConnection>,
    auth: AuthorizedUser) 
    ->  Result<SuccessJsonResponder<String>, FailureJsonResponder<String>>
{
    let res = UserProfile::find()
        .filter(user_profile::Column::Id.eq(auth.user_id))
        .one(db.inner())
        .await?;
    
    match res {
        Some(user) => {
            Ok(BodyData{data: user.username}.into())
        }
        None => return Err(BodyData{data: "User Not Exist".to_string()}.into())
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Stage", |rocket| async {
        rocket.mount("/user", routes![
            user_register, 
            user_login, 
            get_user_info, 
            refresh_token
        ])
    })
}
