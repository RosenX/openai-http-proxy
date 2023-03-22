use serde::{Deserialize, Serialize};

use crate::{Content, FeedProfile, UserContent, UserFeed};

#[derive(Deserialize, Clone, Serialize)]
pub struct FeedContentResponse {
    pub feed_profile: FeedProfile,
    pub contents: Vec<Content>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct CreateFeedResponse {
    pub feed_profile: FeedProfile,
    pub content: Vec<Content>,
    pub user_content: Vec<UserContent>,
    pub user_feed: UserFeed,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct UserFeedResponse {
    pub feed_list: Vec<UserFeed>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct UserContentResponse {
    pub content_list: Vec<UserContent>,
}
