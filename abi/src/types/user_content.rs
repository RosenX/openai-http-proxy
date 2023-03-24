use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, FromRow, Row};

use crate::{Content, FeedProfile, ReadStage, UserContent, UserFeed, UserProfile, DEFAULT_ID};

impl UserFeed {
    pub fn new(user: UserProfile, feed: FeedProfile) -> Self {
        let now_datetime = Utc::now();
        Self {
            id: DEFAULT_ID,
            user_id: user.id,
            feed_id: feed.id,
            name: None,
            logo: None,
            description: None,
            created_time: now_datetime.timestamp(),
            tags: None,
            folder: None,
        }
    }
}

impl UserContent {
    pub fn new(user: UserProfile, content: Content) -> Self {
        Self {
            id: DEFAULT_ID,
            user_id: user.id,
            content_id: content.id,
            stage: ReadStage::Explore as i32, // todo
            tags: None,
            category: None,
            notes: None,
        }
    }
}

impl FromRow<'_, PgRow> for UserContent {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id: i32 = row.get("id");
        let content_id = row.get("content_id");
        let user_id = row.get("user_id");
        let tags = row.get("tags");
        let category = row.get("category");
        let notes = row.get("notes");
        let stage = row.get("stage");
        Ok(UserContent {
            id,
            content_id,
            user_id,
            tags,
            category,
            notes,
            stage,
        })
    }
}

impl FromRow<'_, PgRow> for UserFeed {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id: i32 = row.get("id");
        let feed_id = row.get("feed_id");
        let user_id = row.get("user_id");
        let name = row.get("name");
        let logo = row.get("logo");
        let description = row.get("description");
        let created_time: DateTime<Utc> = row.get("created_time");
        let tags = row.get("tags");
        let folder = row.get("folder");
        Ok(UserFeed {
            id,
            feed_id,
            user_id,
            name,
            logo,
            description,
            created_time: created_time.timestamp(),
            tags,
            folder,
        })
    }
}
