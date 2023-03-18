use rocket::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
    pub url: String,
    pub database: String,
}

use crate::common::errors::InternalError;
use sqlx::{mysql::MySqlPoolOptions, MySql};

pub type MySqlService = sqlx::Pool<MySql>;

impl DatabaseConfig {
    pub fn new() -> Self {
        Config::figment()
            .select("mysql")
            .extract()
            .expect("MySQL配置解析失败")
    }
}

pub async fn setup_database(config: &DatabaseConfig) -> Result<MySqlService, InternalError> {
    let url = format!("{}/{}", config.url, config.database);
    let pool = MySqlPoolOptions::new()
        .max_connections(5) //todo
        .connect(&url)
        .await?;
    Ok(pool)
}
