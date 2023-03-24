use crate::{Content, FeedProfile, InternalError, Url, DEFAULT_ID, SEP, UNKNOWN};
use chrono::{DateTime, Utc};
use feed_rs::model::{Entry, Feed};
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};

use sqlx::{postgres::PgRow, FromRow, Row};

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

impl Content {
    pub fn from_entry(entry: Entry) -> Result<Self, InternalError> {
        let now_datetime = Utc::now();
        let title = match entry.title.to_owned() {
            Some(t) => t.content,
            None => UNKNOWN.to_owned(),
        };
        let content = match entry.content.to_owned() {
            Some(t) => t.body,
            None => None,
        };
        let summary = match entry.summary.to_owned() {
            Some(t) => Some(t.content),
            None => None,
        };
        let link = entry.links.get(0).map(|link| link.href.to_owned());
        let mut md5 = Md5::new();
        md5.update(
            title.clone()
                + content.clone().unwrap_or("".to_string()).as_ref()
                + summary.clone().unwrap_or("".to_string()).as_ref()
                + link.clone().unwrap_or("".to_string()).as_ref(),
        );
        let md5 = hex::encode(md5.finalize());
        let content = Self {
            id: DEFAULT_ID,
            feed_id: None,
            title,
            publish_time: entry.published.map(|t| t.timestamp()),
            authors: Some(
                entry
                    .authors
                    .iter()
                    .map(|p| p.to_owned().name)
                    .collect::<Vec<String>>()
                    .join(SEP),
            ),
            link,
            content,
            summary,
            cover: None,
            summary_algo: None,
            category_algo: None,
            tags_algo: None,
            create_time: now_datetime.timestamp(),
            md5,
        };
        Ok(content)
    }
}

#[derive(Deserialize, Clone, Serialize, FromRow)]
pub struct MD5Wapper {
    pub md5: String,
}

impl FeedProfile {
    pub fn new(feed: &Feed, url: Url) -> Self {
        Self {
            id: DEFAULT_ID,
            url,
            name: match feed.title.to_owned() {
                Some(t) => Some(t.content),
                None => None,
            },
            icon: match feed.icon.to_owned() {
                Some(t) => Some(t.uri),
                None => None,
            },
            logo: match feed.logo.to_owned() {
                Some(t) => Some(t.uri),
                None => None,
            },
            description: match feed.description.to_owned() {
                Some(t) => Some(t.content),
                None => None,
            },
            category_algo: None,
            tags_algo: None,
        }
    }
}
