pub mod user_profile;
pub mod user_feed;
pub mod feed_profile;

use sqlx::{mysql::MySqlPoolOptions, MySql};
use crate::common::errors::InternalError;
use crate::common::config::database::DatabaseConfig;

pub type DatabasePool = sqlx::Pool<MySql>;


pub async fn setup_database(config: &DatabaseConfig) -> Result<DatabasePool, InternalError> {
    let url = format!("{}/{}", config.url, config.database);
    let pool = MySqlPoolOptions::new()
        .max_connections(5) //todo
        .connect(&url)
        .await?;
    Ok(pool)
}