pub mod app_config;
// pub mod responder;

use std::sync::Arc;

use abi::{DatabaseConfig, DbService, InternalError};
pub use app_config::AppConfig;
use content_sync::ContentSyncService;
use tracing::info;

use crate::auth_service::{AuthService, AuthServiceConfig};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub content_service: Arc<ContentSyncService>,
}

// impl FromRef<AppState> for Arc<AuthService> {
//     fn from_ref(app_state: &AppState) -> Arc<AuthService> {
//         app_state.auth_service.clone()
//     }
// }

// impl FromRef<AppState> for Arc<ContentSyncService> {
//     fn from_ref(app_state: &AppState) -> Arc<ContentSyncService> {
//         app_state.content_service.clone()
//     }
// }

impl AppState {
    pub async fn new(
        auth_config: AuthServiceConfig,
        database_config: DatabaseConfig,
    ) -> Result<Self, InternalError> {
        info!("Starting database service from config: {}", database_config);
        let db_service = DbService::from_config(database_config).await?;
        let auth_service = Arc::new(AuthService::new(db_service.clone(), auth_config));
        let content_service = Arc::new(ContentSyncService::new(db_service));
        Ok(Self {
            auth_service,
            content_service,
        })
    }
}
