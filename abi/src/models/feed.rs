use feed_rs::model::Feed;
use serde::{Deserialize, Serialize};

use crate::{DEFAULT_ID, UNKNOWN};

#[derive(Deserialize, Clone, Serialize)]
pub struct FeedProfile {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub icon: Option<String>,
    pub logo: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<String>,
}

impl From<Feed> for FeedProfile {
    fn from(feed: Feed) -> Self {
        Self {
            id: DEFAULT_ID,
            url: feed.links[0].clone().href,
            name: match feed.title {
                Some(t) => t.content,
                None => UNKNOWN.to_owned(),
            },
            icon: match feed.icon {
                Some(t) => Some(t.uri),
                None => None,
            },
            logo: match feed.logo {
                Some(t) => t.uri,
                None => UNKNOWN.to_owned(),
            },
            description: match feed.description {
                Some(t) => Some(t.content),
                None => None,
            },
            category: None,
            tags: None,
        }
    }
}
