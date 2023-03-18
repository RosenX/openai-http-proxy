use crate::{
    common::{errors::InternalError, service::feed_service::FeedService},
    models::request::feed_req::FeedReq,
};
use abi::DbPool;
use feed_rs::model::Feed;

use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FeedProfile {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub icon: Option<String>,
    pub logo: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<String>,
}

impl FeedProfile {
    pub fn new(feed: &Feed, feed_req: FeedReq, config: &FeedService) -> Self {
        Self {
            id: 0,
            url: feed_req.url,
            name: match feed.title.to_owned() {
                Some(t) => t.content,
                None => config.default_name.clone(),
            },
            icon: match feed.icon.to_owned() {
                Some(t) => Some(t.uri),
                None => None,
            },
            logo: match feed.logo.to_owned() {
                Some(t) => t.uri,
                None => config.default_logo.clone(),
            },
            description: match feed.description.to_owned() {
                Some(t) => Some(t.content),
                None => None,
            },
            category: None,
            tags: None,
        }
    }

    pub async fn insert(&mut self, pool: &DbPool) -> Result<Self, InternalError> {
        let feed_id = sqlx::query!(
            r#"
            INSERT INTO feed_profile
                (url, name, logo, icon, description, category, tags)
            VALUES (?,?,?,?,?,?,?)
            "#,
            self.url,
            self.name,
            self.logo,
            self.icon,
            self.description,
            self.category,
            self.tags
        )
        .execute(pool)
        .await?
        .last_insert_id();
        self.id = feed_id as i32;
        Ok(self.to_owned())
    }

    pub async fn find_all(pool: &DbPool) -> Result<Vec<Self>, InternalError> {
        let feeds = sqlx::query_as!(
            FeedProfile,
            r#"
            SELECT * FROM feed_profile
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(feeds)
    }
}