use chrono::{DateTime, Utc};
use feed_rs::model::Entry;
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{InternalError, DEFAULT_ID, SEP, UNKNOWN};

#[derive(Deserialize, Clone, Serialize, FromRow)]
pub struct Content {
    pub id: i32,
    pub feed_id: Option<i32>,
    pub title: String,
    pub cover: Option<String>,
    pub publish_time: Option<DateTime<Utc>>,
    pub authors: Option<String>,
    pub link: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub summary_algo: Option<String>,
    pub category_algo: Option<String>,
    pub tags_algo: Option<String>,
    pub create_time: DateTime<Utc>,
    pub md5: String,
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
        let md5 = String::from_utf8(md5.finalize().to_vec())?;
        let content = Self {
            id: DEFAULT_ID,
            feed_id: None,
            title,
            publish_time: entry.published,
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
            create_time: now_datetime,
            md5,
        };
        Ok(content)
    }
}

#[derive(Deserialize, Clone, Serialize, FromRow)]
pub struct MD5Wapper {
    pub md5: String,
}
