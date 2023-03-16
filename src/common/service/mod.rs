pub mod feed_parser;
pub mod http_service;
pub mod mysql_service;
pub mod jwt_service;

use rocket::{fairing::AdHoc, Config};

use self::{http_service::HttpService, mysql_service::{DatabaseConfig, MySqlService, setup_database}, jwt_service::JsonWebTokenTool};
use super::config::common::CommonConfig;

fn create_config_service() -> CommonConfig {
    Config::figment()
        .select("feed")
        .extract()
        .expect("Feed配置解析失败")
}

fn create_http_service() -> HttpService {
    HttpService::new()
}

pub async fn create_mysql_service() -> MySqlService {
    let config = DatabaseConfig::new();
    setup_database(&config).await.expect("数据库服务启动失败")
}

fn create_jwt_service() -> JsonWebTokenTool {
    JsonWebTokenTool::new()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Service", |rocket| async {
        rocket
            .manage(create_config_service())
            .manage(create_http_service())
            .manage(create_jwt_service())
    })
}
