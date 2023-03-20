use serde::{Deserialize, Serialize};

use crate::{Content, FeedProfile};

#[derive(Deserialize, Clone, Serialize)]
pub struct CreateFeedRequest {
    pub url: String,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct CreateFeedResponse {
    pub feed_profile: FeedProfile,
    pub content: Vec<Content>,
}
