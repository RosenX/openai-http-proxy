use chrono::{DateTime, Utc};
use feed_rs::model::Entry;
use serde::{Deserialize, Serialize};

use crate::{DEFAULT_ID, SEP, UNKNOWN};

#[derive(Deserialize, Clone, Serialize)]
pub struct Content {
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

impl From<Entry> for Content {
    fn from(entry: Entry) -> Self {
        let now_datetime = Utc::now();
        Self {
            id: DEFAULT_ID,
            feed_id: DEFAULT_ID,
            title: match entry.title.to_owned() {
                Some(t) => t.content,
                None => UNKNOWN.to_owned(),
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
                    .join(SEP),
            ),
            link: Some(
                entry
                    .links
                    .iter()
                    .map(|link| link.to_owned().href)
                    .collect::<Vec<String>>()
                    .join(SEP),
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
        }
    }
}
