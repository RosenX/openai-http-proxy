use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, FromRow, Row};

use crate::{Content, UserContent, UserFeed};

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

impl FromRow<'_, PgRow> for Content {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id: i32 = row.get("id");
        let feed_id = row.get("feed_id");
        let title = row.get("title");
        let cover = row.get("cover");
        let publish_time = row.get("publish_time");
        let content = row.get("content");
        let authors = row.get("author");
        let link = row.get("link");
        let summary = row.get("summary");
        let summary_algo = row.get("summary_algo");
        let tags_algo = row.get("tag_algo");
        let category_algo = row.get("category_algo");
        let create_time: DateTime<Utc> = row.get("created_time");
        let md5 = row.get("md5");
        Ok(Content {
            id,
            feed_id,
            title,
            cover,
            publish_time,
            content,
            authors,
            link,
            summary,
            summary_algo,
            tags_algo,
            category_algo,
            create_time: create_time.timestamp(),
            md5,
        })
    }
}
