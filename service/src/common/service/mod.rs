pub mod mysql_service;

use abi::DbPool;
use content_service::ContentService;

use rocket::{fairing::AdHoc, Config};
use user_service::UserService;

use crate::auth_service::AuthService;

use self::mysql_service::{setup_database, DatabaseConfig};

pub async fn create_mysql_service() -> DbPool {
    let config = DatabaseConfig::new();
    setup_database(&config).await.expect("数据库服务启动失败")
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Service", |rocket| async {
        let mysql_service = create_mysql_service().await;

        let auth_config = Config::figment()
            .select("auth_service")
            .extract()
            .expect("auth配置解析失败");

        rocket
            .manage(ContentService::new(mysql_service.clone()))
            .manage(UserService::new(mysql_service.clone()))
            .manage(AuthService::new(mysql_service, auth_config))
    })
}

// pub fn backgroupd_job() -> AdHoc {
//     AdHoc::on_liftoff("Background Fetch Feed", |_| {
//         Box::pin(async move {
//             let mysql_service = create_mysql_service().await;
//             let feed_service = create_feed_service();
//             rocket::tokio::spawn(async move {
//                 let mut interval = tokio::time::interval(time::Duration::from_secs(60));
//                 loop {
//                     let feed_list = feed_service.fetch_all_feed(&mysql_service).await.unwrap();
//                     println!("feed length {}", feed_list.len());
//                     interval.tick().await;
//                 }
//             });
//         })
//     })
// }
