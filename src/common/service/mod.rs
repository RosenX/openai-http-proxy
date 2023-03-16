pub mod feed_service;
pub mod http_service;
pub mod mysql_service;
pub mod jwt_service;

use rocket::{fairing::AdHoc};

use self::{http_service::HttpService, mysql_service::{DatabaseConfig, MySqlService, setup_database}, jwt_service::JwtService, feed_service::FeedService};

fn create_feed_service() -> FeedService {
    FeedService::new()
}

fn create_http_service() -> HttpService {
    HttpService::new()
}

pub async fn create_mysql_service() -> MySqlService {
    let config = DatabaseConfig::new();
    setup_database(&config).await.expect("数据库服务启动失败")
}

fn create_jwt_service() -> JwtService {
    JwtService::new()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Service", |rocket| async {
        rocket
            .manage(create_feed_service())
            .manage(create_http_service())
            .manage(create_jwt_service())
    })
}
