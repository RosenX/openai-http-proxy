mod common;
mod database;
mod models;
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
        .attach(common::service::stage())
        .attach(routes::user::stage())
        .attach(routes::feed::stage())
        .attach(routes::post::stage())
        .attach(common::catcher::stage())
        .attach(common::service::backgroupd_job())
}
