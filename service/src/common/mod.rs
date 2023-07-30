pub mod app_config;
pub mod auth;
// pub mod responder;

use std::sync::Arc;

use abi::{DatabaseConfig, DbService, InternalError};
pub use app_config::AppConfig;
pub use auth::AuthorizedUser;
use content_sync::ContentSyncService;
use tracing::info;

use app_config::AuthingConfig;

#[derive(Clone)]
pub struct AppState {
    pub content_service: Arc<ContentSyncService>,
    pub authing: Arc<AuthingConfig>,
}

impl AppState {
    pub async fn new(
        auth_config: AuthingConfig,
        database_config: DatabaseConfig,
    ) -> Result<Self, InternalError> {
        info!("Starting database service from config: {}", database_config);
        let db_service = DbService::from_config(database_config).await?;
        let content_service = Arc::new(ContentSyncService::new(db_service));
        Ok(Self {
            content_service,
            authing: Arc::new(auth_config),
        })
    }
}
