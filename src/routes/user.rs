use crate::entities::{prelude::*, user_profile};
use crate::utils::CommonResponse;
use rocket::fairing::AdHoc;
use rocket::serde::{Deserialize};
use rocket::serde::json::{Json};
use rocket::{post, State, routes};
use chrono::{Local};
use sea_orm::*;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserRegister {
    username: String,
    email: String,
    password: String
}

#[post("/user/create", data = "<user>")]
async fn user_register(user: Json<UserRegister>, db: DatabaseConnection) -> Result<CommonResponse> {
    let email = user.into_inner().email;
    let password = user.into_inner().password;
    let username = user.into_inner().username;

    let now_datetime = Local::now().naive_local();
    println!("{}", now_datetime);

    let user = user_profile::ActiveModel {
        username: ActiveValue::Set(username),
        email: ActiveValue::Set(email),
        hash_password: ActiveValue::Set(password),
        created_time: ActiveValue::Set(now_datetime),
        ..Default::default()
    };
    UserProfile::insert(user).exec(&db).await?;
    Ok(CommonResponse::SUCCESS)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Stage", |rocket| async {
        rocket.mount("/user", routes![user_register])
    })
}
