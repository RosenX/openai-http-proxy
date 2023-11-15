use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserServiceConfig {
    pub app_store_password: String,
}
