pub mod feed_service;
pub mod http_service;
pub mod jwt_service;
pub mod mysql_service;

use std::time::{self};

use rocket::fairing::AdHoc;

use self::{
    feed_service::FeedService,
    http_service::HttpService,
    jwt_service::JwtService,
    mysql_service::{setup_database, DatabaseConfig, MySqlService},
};

pub fn create_feed_service() -> FeedService {
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
        let mysql_service = create_mysql_service().await;
        rocket
            .manage(create_feed_service())
            .manage(create_http_service())
            .manage(create_jwt_service())
            .manage(mysql_service)
    })
}

pub fn backgroupd_job() -> AdHoc {
    AdHoc::on_liftoff("Background Fetch Feed", |_| {
        Box::pin(async move {
            let mysql_service = create_mysql_service().await;
            let feed_service = create_feed_service();
            rocket::tokio::spawn(async move {
                let mut interval = tokio::time::interval(time::Duration::from_secs(60));
                loop {
                    let feed_list = feed_service.fetch_all_feed(&mysql_service).await.unwrap();
                    println!("feed length {}", feed_list.len());
                    interval.tick().await;
                }
            });
        })
    })
}
