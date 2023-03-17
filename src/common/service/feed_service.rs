use feed_rs::{model::Feed, parser};
use rocket::{serde::Deserialize, Config};
use tokio::time::sleep;
use std::time;

use crate::{common::errors::InternalError, database::feed_profile::FeedProfile};

use super::{http_service::HttpService, mysql_service::MySqlService};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FeedService {
    pub default_name: String,
    pub default_logo: String,
    pub default_title: String,
    pub default_seq: String,
}

impl FeedService {
    pub fn new() -> Self {
        Config::figment()
            .select("feed")
            .extract()
            .expect("Feed配置解析失败")
    }

    pub async fn fetch_from_url(
        http_service: &HttpService,
        url: &String,
    ) -> Result<Feed, InternalError> {
        let data = http_service.get(url).await?;
        let feed = parser::parse(data.as_bytes())?;
        Ok(feed)
    }

    pub async fn create_cron_job(&self, pool: &MySqlService) -> Result<(), InternalError> {
        loop {
            let mut interval = tokio::time::interval(time::Duration::from_secs(60));
            let feed_list = self.fetch_all_feed(&pool).await?;
            println!("feed length {}", feed_list.len());
            interval.tick().await;
        }
    }

    pub async fn fetch_all_feed(
        &self,
        pool: &MySqlService,
    ) -> Result<Vec<FeedProfile>, InternalError> {
        let feed_list = FeedProfile::find_all(pool).await?;
        Ok(feed_list)
    }
}
