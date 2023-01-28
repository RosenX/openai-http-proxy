mod entities;
mod database;
mod routes;
mod utils;
#[cfg(test)] mod test;

use rocket::{launch, Config};
use database::{setup_database, DatabaseConfig};
use routes::authorization::{JsonWebTokenConfig};


#[launch]
async fn rocket_app() -> _ {
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
        .attach(routes::source::stage())
        .attach(routes::content::stage())
}