mod entities;
mod database;
mod routes;
mod utils;
#[cfg(test)] mod test;

use env_logger::Env;
use log::{info, warn, debug, error};
use rocket::{launch, Config};
use database::{setup_database, DatabaseConfig};
use routes::authorization::{JsonWebTokenConfig};


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

    let jwt_config: JsonWebTokenConfig = Config::figment()
        .select("jsonwebtoken")
        .extract()
        .expect("jsonwebtoken配置解析失败");

    

    error!("Bright red error");
    info!("This only appears in the log file");
    warn!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
    
    rocket
        .manage(db)
        .manage(jwt_config)
        .attach(routes::user::stage())
        .attach(routes::source::stage())
        .attach(routes::content::stage())
}