use rocket_db_pools::{sqlx, Database};
use rocket::fairing::{self, AdHoc};
use rocket::{Rocket, Build};


#[derive(Database)]
#[database("feed_inbox")]
pub struct MysqlConnection(sqlx::MySqlPool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match MysqlConnection::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/sqlx/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        }
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(MysqlConnection::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}