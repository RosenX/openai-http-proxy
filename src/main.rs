mod entities;
mod database;
mod routes;
mod utils;

use rocket::{launch};
use database::setup_database;


#[launch]
async fn rocket() -> _ {
    let db = match setup_database().await {
        Ok(db) => db,
        Err(e) => panic!("{}", e),
    };
    
    rocket::build()
        .manage(db)
        .attach(routes::user::stage())
}