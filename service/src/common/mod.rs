pub mod app_config;
pub mod auth;
// pub mod responder;

use std::sync::Arc;

use abi::{DatabaseConfig, DbService, InternalError, UserServiceConfig};
pub use app_config::AppConfig;
pub use auth::AuthorizedUser;
use content_sync::ContentSyncService;
use tracing::info;

use app_config::AuthingConfig;
use user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub content_service: Arc<ContentSyncService>,
    pub user_service: Arc<UserService>,
    pub authing: Arc<AuthingConfig>,
}

impl AppState {
    pub async fn new(
        auth_service_config: AuthingConfig,
        user_service_config: UserServiceConfig,
        database_config: DatabaseConfig,
    ) -> Result<Self, InternalError> {
        info!("Starting database service from config: {}", database_config);
        let db = DbService::from_config(database_config).await?;
        let content_service = Arc::new(ContentSyncService::new(db.clone()));
        let user_service = Arc::new(UserService::new(db, user_service_config));
        Ok(Self {
            content_service,
            user_service,
            authing: Arc::new(auth_service_config),
        })
    }
}
