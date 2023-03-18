use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
