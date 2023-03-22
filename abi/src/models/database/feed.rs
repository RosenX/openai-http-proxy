use feed_rs::model::Feed;
use serde::{Deserialize, Serialize};

use crate::{Url, DEFAULT_ID};

#[derive(Deserialize, Clone, Serialize)]
pub struct FeedProfile {
    pub id: i32,
    pub url: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub category_algo: Option<String>,
    pub tags_algo: Option<String>,
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
