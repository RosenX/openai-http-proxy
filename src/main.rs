mod entities;
mod database;
mod routes;
mod utils;
#[cfg(test)] mod test;

use env_logger::Env;
use rocket::{launch, Config};
use database::{setup_database, DatabaseConfig};
use routes::authorization::JsonWebTokenTool;


#[launch]
async fn rocket_app() -> _ {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "Info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let rocket = rocket::build();
    let mysql_config: DatabaseConfig = Config::figment().select("mysql").extract().expect("MySQL配置解析失败");

    let db = match setup_database(&mysql_config).await {
        Ok(db) => db,
        Err(e) => panic!("{}", e),
    };

    let jwt: JsonWebTokenTool = Config::figment()
        .select("jsonwebtoken")
        .extract()
        .expect("jsonwebtoken配置解析失败");

    rocket
        .manage(db)
        .manage(jwt)
        .attach(routes::user::stage())
        .attach(routes::source::stage())
        .attach(routes::content::stage())
}