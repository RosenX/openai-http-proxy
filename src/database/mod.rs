pub mod user_profile;
pub mod user_feed;

use rocket::{serde::Deserialize};
use sqlx::{mysql::MySqlPoolOptions, MySql};

use crate::common::errors::InternalError;

pub type DatabasePool = sqlx::Pool<MySql>;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
    pub url: String,
    pub database: String,
}

pub async fn setup_database(config: &DatabaseConfig) -> Result<DatabasePool, InternalError> {
    let url = format!("{}/{}", config.url, config.database);
    let pool = MySqlPoolOptions::new()
        .max_connections(5) //todo
        .connect(&url)
        .await?;
    Ok(pool)
}