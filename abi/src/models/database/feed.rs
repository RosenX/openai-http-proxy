use feed_rs::model::Feed;

use crate::{FeedProfile, Url, DEFAULT_ID};

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
