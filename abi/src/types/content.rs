use crate::{Feed, FeedGroup, FeedItem, FeedUpdateRecord};
use sqlx::{postgres::PgRow, FromRow, Row};

impl FromRow<'_, PgRow> for FeedGroup {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("group_id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            update_time: row.try_get("update_time")?,
        })
    }
}

impl FromRow<'_, PgRow> for FeedItem {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("item_id")?,
            is_focus: row.try_get("is_focus")?,
            is_seen: row.try_get("is_seen")?,
            title: row.try_get("title")?,
            cover: row.try_get("cover")?,
            link: row.try_get("link")?,
            publish_time: row.try_get("publish_time")?,
            authors: row.try_get("authors")?,
            tags: row.try_get("tags")?,
            category: row.try_get("category")?,
            description: row.try_get("description")?,
            summary_algo: row.try_get("summary_algo")?,
            content: row.try_get("content")?,
            content_have_parsed: row.try_get("content_have_parsed")?,
            create_time: row.try_get("create_time")?,
            md5_string: row.try_get("md5_string")?,
            feed_id: row.try_get("feed_id")?,
            update_time: row.try_get("update_time")?,
        })
    }
}

impl FromRow<'_, PgRow> for Feed {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("feed_id")?,
            url: row.try_get("url")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            custom_description: row.try_get("custom_description")?,
            custom_name: row.try_get("custom_name")?,
            group_id: row.try_get("group_id")?,
            update_time: row.try_get("update_time")?,
            logo: row.try_get("logo")?,
            custom_logo: row.try_get("custom_logo")?,
            create_time: row.try_get("create_time")?,
            feed_type: row.try_get("feed_type")?,
            tags: row.try_get("tags")?,
        })
    }
}

impl FromRow<'_, PgRow> for FeedUpdateRecord {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            feed_id: row.try_get("feed_id")?,
            update_time: row.try_get("update_time")?,
            last_content_hash: row.try_get("last_content_hash")?,
            last_update: row.try_get("last_update")?,
            last_item_publish_time: row.try_get("last_item_publish_time")?,
        })
    }
}
