use crate::{
    common::{
        config::common::CommonConfig, errors::InternalError
    },
    database::DatabasePool,
    models::request::feed_req::FeedReq,
};
use feed_rs::{
    model::{Entry, Feed},
    parser,
};
use futures::SinkExt;
use rocket::serde::{Deserialize, Serialize};

use super::feed_post::FeedPost;

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
    pub async fn new(feed: Feed, feed_req: FeedReq, config: &CommonConfig) -> Result<Self, InternalError> {
        let feed_info = Self {
            id: 0,
            url: feed_req.url,
            name: match feed.title {
                Some(t) => t.content,
                None => config.default_name.clone(),
            },
            icon: match feed.icon {
                Some(t) => Some(t.uri),
                None => None,
            },
            logo: match feed.logo {
                Some(t) => t.uri,
                None => config.default_logo.clone(),
            },
            description: match feed.description {
                Some(t) => Some(t.content),
                None => None,
            },
            category: None,
            tags: None,
        };
        Ok(feed_info)
    }

    pub async fn create_feed(&mut self, pool: &DatabasePool) -> Result<Self, InternalError> {
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
}
