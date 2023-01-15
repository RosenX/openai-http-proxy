use crate::database::DbErrorResponder;
use crate::utils::{prelude::*, SUCCESS};
use crate::entities::{prelude::*, user_profile};
use rocket::fairing::AdHoc;
use rocket::serde::{Deserialize};
use rocket::serde::json::{Json};
use rocket::{post, State, routes};
use chrono::{Local};
use sea_orm::*;

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
    ->  Result<SuccessJsonResponder, DbErrorResponder>
{
    let info = info.into_inner();

    let now_datetime = Local::now().naive_local();
    println!("{}", now_datetime);

    let user = user_profile::ActiveModel {
        username: ActiveValue::Set(info.username),
        email: ActiveValue::Set(info.email),
        hash_password: ActiveValue::Set(info.password),
        created_time: ActiveValue::Set(now_datetime),
        ..Default::default()
    };
    
    UserProfile::insert(user).exec(db.inner()).await?;
    Ok(StringBodyData{data: SUCCESS.to_string()}.into())
}

#[post("/", data = "<info>")]
async fn user_login(info: Json<LoginInfo>, db: &State<DatabaseConnection>) 
    ->  Result<SuccessJsonResponder, DbErrorResponder>
{
    let info = info.into_inner();

    let res = UserProfile::find()
        .filter(user_profile::Column::Email.eq(info.email))
        .filter(user_profile::Column::HashPassword.eq(info.password))
        .one(db.inner())
        .await?;
    
    Ok(StringBodyData{data: res.unwrap().username}.into())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Stage", |rocket| async {
        rocket.mount("/user", routes![user_register, user_login])
    })
}
