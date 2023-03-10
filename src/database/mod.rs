pub mod user_profile;
pub mod user_custom_feed;
pub mod user_custom_post;
pub mod feed_profile;
pub mod feed_post;

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