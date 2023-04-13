use std::ops::Deref;

use serde::Deserialize;

use crate::InternalError;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub database: String,
    pub max_connection: u32,
}

pub type DbOption = sqlx::postgres::PgPoolOptions;
pub type DbPool = sqlx::PgPool;

#[derive(Clone)]
pub struct DbService(DbPool);

impl AsRef<DbPool> for DbService {
    fn as_ref(&self) -> &DbPool {
        &self.0
    }
}

impl Deref for DbService {
    type Target = DbPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DbService {
    pub async fn from_config(config: DatabaseConfig) -> Result<Self, InternalError> {
        let url = format!("{}/{}", config.url, config.database);
        let pool = DbOption::new()
            .max_connections(config.max_connection) //todo
            .connect(&url)
            .await?;
        Ok(Self(pool))
    }
}
