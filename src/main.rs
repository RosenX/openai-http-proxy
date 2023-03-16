mod common;
mod database;
mod models;
mod routes;

use common::service::{create_mysql_service};
use env_logger::Env;
use rocket::{launch};

#[launch]
async fn rocket_app() -> _ {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "Info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let db_service = create_mysql_service().await;

    let rocket = rocket::build();

    rocket
        .manage(db_service)
        .attach(common::service::stage())
        .attach(routes::user::stage())
        .attach(routes::feed::stage())
        .attach(routes::post::stage())
        .attach(common::catcher::stage())
}
