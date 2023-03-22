pub mod responder;

use abi::{DatabaseConfig, DbService};
use content_service::ContentService;
pub use responder::{ErrorInfo, ErrorResponse, SuccessResponse};
use rocket::{fairing::AdHoc, Config};
use user_service::UserService;

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

pub fn create_user_service(db_service: DbService) -> UserService {
    UserService::new(db_service)
}

pub fn create_content_service(db_service: DbService) -> ContentService {
    ContentService::new(db_service)
}

pub fn create_backgroup_job() -> AdHoc {
    AdHoc::on_liftoff("Background Fetch Feed", |_| {
        Box::pin(async move {
            let db_service = create_mysql_service().await;
            let content_service = create_content_service(db_service);
            content_service.start_fetch_content();
        })
    })
}

pub fn init_service() -> AdHoc {
    AdHoc::on_ignite("Loading Service", |rocket| async {
        let db_service = create_mysql_service().await;

        rocket
            .manage(create_content_service(db_service.clone()))
            .manage(create_user_service(db_service.clone()))
            .manage(create_auth_service(db_service))
            .attach(create_backgroup_job())
    })
}
