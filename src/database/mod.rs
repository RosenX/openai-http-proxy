use rocket::serde::Deserialize;

mod setup_database;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
    pub url: String,
    pub database: String,
}

pub use setup_database::setup_database;