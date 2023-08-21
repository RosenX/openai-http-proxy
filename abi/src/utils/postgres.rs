use std::{fmt::Display, ops::Deref};

use serde::Deserialize;
use sqlx::ConnectOptions;
use tracing::log::LevelFilter;
use utoipa::ToSchema;

use crate::InternalError;

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub max_connection: u32,
}

impl Display for DatabaseConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{host: {}, port: {}, user: {}, database: {}, max_connection: {}}}",
            self.host, self.port, self.user, self.database, self.max_connection
        )
    }
}

pub type DbOption = sqlx::postgres::PgPoolOptions;
pub type DbConnectOption = sqlx::postgres::PgConnectOptions;
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
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.user, config.password, config.host, config.port, config.database
        );
        let connect_options = url
            .parse::<DbConnectOption>()
            .unwrap()
            .log_statements(LevelFilter::Trace)
            .to_owned();

        let pool = DbOption::new()
            .max_connections(config.max_connection)
            .connect_with(connect_options)
            .await
            .map_err(|e| InternalError::DatabaseStartError(e.to_string()))?;

        Ok(Self(pool))
    }
}
