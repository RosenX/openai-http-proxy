use crate::common::service::feed_service::FeedService;
use abi::{DbPool, InternalError};
use chrono::{DateTime, Utc};
use feed_rs::model::Entry;
use rocket::serde::{Deserialize, Serialize};

use super::feed_profile::FeedProfile;

#[derive(Deserialize, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FeedPost {
    pub id: i32,
    pub feed_id: i32,
    pub title: String,
    pub cover: Option<String>,
    pub publish_time: DateTime<Utc>,
    pub authors: Option<String>,
    pub link: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub summary_algo: Option<String>,
    pub category_algo: Option<String>,
    pub tags_algo: Option<String>,
}

impl FeedPost {
    pub fn new(entry: &Entry, feed_profile: &FeedProfile, config: &FeedService) -> Self {
        let now_datetime = Utc::now();
        let post = Self {
            id: 0,
            feed_id: feed_profile.id,
            title: match entry.title.to_owned() {
                Some(t) => t.content,
                None => config.default_title.clone(),
            },
            publish_time: match entry.published {
                Some(t) => t,
                None => now_datetime,
            },
            authors: Some(
                entry
                    .authors
                    .iter()
                    .map(|p| p.to_owned().name)
                    .collect::<Vec<String>>()
                    .join(config.default_seq.as_ref()),
            ),
            link: Some(
                entry
                    .links
                    .iter()
                    .map(|link| link.to_owned().href)
                    .collect::<Vec<String>>()
                    .join(config.default_seq.as_str()),
            ),
            content: match entry.content.to_owned() {
                Some(t) => t.body,
                None => None,
            },
            summary: match entry.summary.to_owned() {
                Some(t) => Some(t.content),
                None => None,
            },
            cover: None,
            summary_algo: None,
            category_algo: None,
            tags_algo: None,
        };
        post
    }

    pub async fn insert(&mut self, pool: &DbPool) -> Result<Self, InternalError> {
        let post_id = sqlx::query!(
            r#"
            INSERT INTO feed_post (
                feed_id,
                title,
                publish_time,
                cover,
                authors,
                link,
                content,
                summary,
                summary_algo,
                category_algo,
                tags_algo
            )
            VALUES (?,?,?,?,?,?,?,?,?,?,?)
            "#,
            self.feed_id,
            self.title,
            self.publish_time,
            self.cover,
            self.authors,
            self.link,
            self.content,
            self.summary,
            self.summary_algo,
            self.category_algo,
            self.tags_algo
        )
        .execute(pool)
        .await?
        .last_insert_id();
        self.id = post_id as i32;
        Ok(self.to_owned())
    }
}
