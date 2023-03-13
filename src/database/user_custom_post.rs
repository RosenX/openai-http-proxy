use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;
use rocket::serde::Serialize;
use sqlx::FromRow;

use crate::common::errors::InternalError;
use crate::routes::authorization::AuthorizedUser;


// CREATE TABLE IF NOT EXISTS user_custom_post (
//     post_id INT NOT NULL COMMENT 'Feed ID',
//     user_id INT NOT NULL COMMENT 'User ID',
//     tags_algo VARCHAR(100) COMMENT '算法标签',
//     category_algo VARCHAR(100) COMMENT '算法分类',
//     notes TEXT COMMENT '用户笔记',
//     PRIMARY KEY (user_id, post_id),
//     FOREIGN KEY (user_id) REFERENCES user_profile(id),
//     FOREIGN KEY (post_id) REFERENCES feed_post(id)
// );

use super::DatabasePool;
use super::feed_post::FeedPost;

#[derive(Clone, Debug, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCustomPost {
    pub user_id: i32,
    pub feed_id: i32,
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

impl UserCustomPost {
    pub fn new(feed_post: FeedPost, user: AuthorizedUser) -> Self {
        Self {
            user_id:  user.id,
            feed_id: feed_post.feed_id,
            post_id: feed_post.id,
            link: feed_post.link,
            content: feed_post.content,
            title: feed_post.title,
            authors: feed_post.authors,
            tags: feed_post.tags_algo,
            category: feed_post.category_algo,
            notes: None,
            publish_time: feed_post.publish_time,
        }
    }

    pub async fn insert(&self, pool: &DatabasePool) -> Result<(), InternalError> {
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
        pool: &DatabasePool,
        user_id: i32,
        lastest_post_id: i32
    ) -> Result<Vec<UserCustomPost>, InternalError> {
        let res = sqlx::query_as!(
            UserCustomPost,
            r#"
            SELECT 
                user_id,
                feed_id,
                post_id,
                link,
                content,
                title,
                authors,
                IF (up.tags IS NULL, fp.tags_algo, up.tags) as tags,
                IF (up.category IS NULL, fp.category_algo, up.category) as category,
                notes,
                publish_time
            FROM user_custom_post up
            JOIN feed_post  fp
            ON up.post_id = fp.id
            WHERE user_id = ? AND post_id > ?
            "#,
            user_id,
            lastest_post_id
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }

    pub async fn retrieve_lastest_post_by_id(
        pool: &DatabasePool,
        user_id: i32,
        lastest_post_id: i32,
        feed_id: i32
    ) -> Result<Vec<UserCustomPost>, InternalError> {
        let res = sqlx::query_as!(
            UserCustomPost,
            r#"
            SELECT 
                user_id,
                feed_id,
                post_id,
                link,
                content,
                title,
                authors,
                IF (up.tags IS NULL, fp.tags_algo, up.tags) as tags,
                IF (up.category IS NULL, fp.category_algo, up.category) as category,
                notes,
                publish_time
            FROM user_custom_post up
            JOIN feed_post  fp
            ON up.post_id = fp.id
            WHERE 
                user_id = ? 
                AND post_id > ? 
                AND feed_id = ?
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