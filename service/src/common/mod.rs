pub mod responder;

use abi::{DatabaseConfig, DbService};
use content_sync::ContentSyncService;
pub use responder::{ErrorInfo, ErrorResponse, SuccessResponse};
use rocket::{fairing::AdHoc, Config};

use crate::auth_service::AuthService;

pub async fn create_mysql_service() -> DbService {
    let config: DatabaseConfig = Config::figment()
        .select("database")
        .extract()
        .expect("数据库配置解析失败");
    DbService::from_config(config)
        .await
        .expect("数据库连接无法建立")
}

pub fn create_auth_service(db_service: DbService) -> AuthService {
    let config = Config::figment()
        .select("auth_service")
        .extract()
        .expect("auth_service配置解析失败");
    AuthService::new(db_service, config)
}

pub fn create_content_sync_service(db_service: DbService) -> ContentSyncService {
    ContentSyncService::new(db_service)
}

pub fn init_service() -> AdHoc {
    AdHoc::on_ignite("Loading Service", |rocket| async {
        let db_service = create_mysql_service().await;

        rocket
            .manage(create_content_sync_service(db_service.clone()))
            .manage(create_auth_service(db_service))
    })
}
