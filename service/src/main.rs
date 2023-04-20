#![deny(unused_crate_dependencies)]
mod auth_service;
mod common;
mod routes;

use env_logger::Env;
use rocket::launch;

#[launch]
async fn rocket_app() -> _ {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "Info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    rocket::build()
        .attach(common::init_service())
        .attach(routes::api())
}
