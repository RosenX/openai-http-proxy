use crate::database::ErrorResponder;
use crate::entities::{prelude::*, user_profile};
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
async fn user_register(user: Json<UserRegister>, db: &State<DatabaseConnection>) 
    ->  Result<(), ErrorResponder> 
{
    let user = user.into_inner();

    let now_datetime = Local::now().naive_local();
    println!("{}", now_datetime);

    let user = user_profile::ActiveModel {
        username: ActiveValue::Set(user.username),
        email: ActiveValue::Set(user.email),
        hash_password: ActiveValue::Set(user.password),
        created_time: ActiveValue::Set(now_datetime),
        ..Default::default()
    };

    UserProfile::insert(user).exec(db.inner()).await?;
    Ok(())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Stage", |rocket| async {
        rocket.mount("/", routes![user_register])
    })
}
