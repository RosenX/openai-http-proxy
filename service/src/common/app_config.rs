use crate::auth_service::AuthServiceConfig;
use abi::DatabaseConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub auth_service: AuthServiceConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}
