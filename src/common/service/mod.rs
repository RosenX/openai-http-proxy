use rocket::{fairing::AdHoc, Config};

use self::http_service::HttpService;

use super::config::common::CommonConfig;

pub mod feed_parser;
pub mod http_service;

fn create_config_service() -> CommonConfig {
    let common_config: CommonConfig = Config::figment()
        .select("feed")
        .extract()
        .expect("Feed配置解析失败");
    common_config
}

fn create_http_service() -> HttpService {
    let http = HttpService::new();
    http
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Service", |rocket| async {
        rocket
            .manage(create_config_service())
            .manage(create_http_service())
    })
}
