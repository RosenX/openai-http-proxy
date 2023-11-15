use abi::{DatabaseConfig, UserServiceConfig};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub authing: AuthingConfig,
    pub user_service: UserServiceConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct AuthingConfig {
    pub app_secret: String,
}
