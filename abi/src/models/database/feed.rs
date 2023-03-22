use feed_rs::model::Feed;
use serde::{Deserialize, Serialize};

use crate::DEFAULT_ID;

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

impl From<Feed> for FeedProfile {
    fn from(feed: Feed) -> Self {
        Self {
            id: DEFAULT_ID,
            url: feed.links[0].clone().href,
            name: match feed.title {
                Some(t) => Some(t.content),
                None => None,
            },
            icon: match feed.icon {
                Some(t) => Some(t.uri),
                None => None,
            },
            logo: match feed.logo {
                Some(t) => Some(t.uri),
                None => None,
            },
            description: match feed.description {
                Some(t) => Some(t.content),
                None => None,
            },
            category_algo: None,
            tags_algo: None,
        }
    }
}
