use abi::{DbPool, InternalError};
use rocket::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
    pub url: String,
    pub database: String,
}

use sqlx::postgres::PgPoolOptions;

impl DatabaseConfig {
    pub fn new() -> Self {
        Config::figment()
            .select("mysql")
            .extract()
            .expect("数据库配置解析失败")
    }
}

pub async fn setup_database(config: &DatabaseConfig) -> Result<DbPool, InternalError> {
    let url = format!("{}/{}", config.url, config.database);
    let pool = PgPoolOptions::new()
        .max_connections(5) //todo
        .connect(&url)
        .await?;
    Ok(pool)
}
