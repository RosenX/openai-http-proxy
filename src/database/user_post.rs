use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;
use rocket::serde::Serialize;
use sqlx::FromRow;

use crate::common::errors::InternalError;
use crate::common::service::mysql_service::MySqlService;

#[derive(Clone, Debug, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserPost {
    pub user_id: i32,
    pub feed_id: i32,
    pub feed_name: Option<String>,
    pub cover: Option<String>,
    pub stage: Option<i64>,
    pub post_id: i32,
    pub link: Option<String>,
    pub content: Option<String>,
    pub title: String,
    pub authors: Option<String>,
    pub tags: Option<String>,
    pub category: Option<String>,
    pub notes: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub publish_time: DateTime<Utc>,
}

impl UserPost {
    pub async fn insert(&self, pool: &MySqlService) -> Result<(), InternalError> {
        sqlx::query!(
            r#"
            INSERT INTO user_custom_post (
                post_id, user_id, tags, category, notes
            ) VALUES (?,?,?,?,?)
            "#,
            self.post_id,
            self.user_id,
            self.tags,
            self.category,
            self.notes,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn retrieve_lastest_post(
        pool: &MySqlService,
        user_id: i32,
        lastest_post_id: i32
    ) -> Result<Vec<UserPost>, InternalError> {
        let res = sqlx::query_as!(
            UserPost,
            r#"
            SELECT 
                ucf.user_id,
                ucf.feed_id,
                ucf.name as feed_name,
                fp.cover,
                IF (ucp.stage IS NULL, 1, ucp.stage) as stage,
                fp.id as post_id,
                fp.link,
                fp.content,
                fp.title,
                fp.authors,
                IF (ucp.tags IS NULL, fp.tags_algo, ucp.tags) as tags,
                IF (ucp.category IS NULL, fp.category_algo, ucp.category) as category,
                ucp.notes,
                fp.publish_time
            FROM user_custom_feed ucf
            JOIN feed_post fp ON ucf.feed_id = fp.feed_id
            LEFT JOIN user_custom_post ucp ON ucf.user_id = ucp.user_id
            WHERE ucf.user_id = ? AND fp.id > ?
            "#,
            user_id,
            lastest_post_id
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }

    pub async fn retrieve_lastest_post_by_id(
        pool: &MySqlService,
        user_id: i32,
        lastest_post_id: i32,
        feed_id: i32
    ) -> Result<Vec<UserPost>, InternalError> {
        let res = sqlx::query_as!(
            UserPost,
            r#"
            SELECT 
                ucf.user_id,
                ucf.feed_id,
                ucf.name as feed_name,
                fp.cover,
                IF (ucp.stage IS NULL, 1, ucp.stage) as stage,
                fp.id as post_id,
                fp.link,
                fp.content,
                fp.title,
                fp.authors,
                IF (ucp.tags IS NULL, fp.tags_algo, ucp.tags) as tags,
                IF (ucp.category IS NULL, fp.category_algo, ucp.category) as category,
                ucp.notes,
                fp.publish_time
            FROM user_custom_feed ucf
            JOIN feed_post fp ON ucf.feed_id = fp.feed_id
            LEFT JOIN user_custom_post ucp ON ucf.user_id = ucp.user_id
            WHERE ucf.user_id = ? AND fp.id > ? AND ucf.feed_id = ?
            "#,
            user_id,
            lastest_post_id,
            feed_id
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
}