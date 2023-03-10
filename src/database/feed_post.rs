use crate::{
    common::{
        config::common::CommonConfig, errors::InternalError, },
    database::DatabasePool,
    models::request::feed_req::FeedReq,
};
use chrono::{DateTime, Utc};
use feed_rs::{parser, model::Entry};
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FeedPost {
    pub id: i32,
    pub feed_id: i32,
    pub title: String,
    pub publish_time: DateTime<Utc>,
    pub authors: Option<String>,
    pub link: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub summary_algo: Option<String>,
    pub category_algo: Option<String>,
    pub tags_algo: Option<String>,
}