mod entities;
mod database;
mod routes;
mod utils;
mod config;

use config::DatabaseConfig;
use rocket::{launch, Config, fairing::AdHoc};
use database::setup_database;
use routes::authorization::{JsonWebTokenConfig, JsonWebTokenTool};


#[launch]
async fn rocket() -> _ {
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
    
    rocket
        .manage(db)
        .manage(jwt_config)
        .attach(routes::user::stage())
}