use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;
use rocket::serde::Serialize;
use sqlx::FromRow;

use crate::{common::errors::InternalError, routes::{authorization::AuthorizedUser}};

use super::{DatabasePool, feed_profile::FeedProfile};

#[derive(Clone, Debug, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserFeed {
    pub user_id: i32,
    pub feed_id: i32,
    pub url: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub created_time: DateTime<Utc>,
}

impl UserFeed {
    pub fn new(feed_profile: FeedProfile, user: AuthorizedUser) -> Self {
        let now_datetime = Utc::now();
        Self {
            user_id:  user.id,
            feed_id: feed_profile.id,
            url: feed_profile.url,
            name: Some(feed_profile.name),
            icon: feed_profile.icon,
            logo: Some(feed_profile.logo),
            description: feed_profile.description,
            created_time: now_datetime
        }
    }

    pub async fn insert(&self, pool: &DatabasePool) -> Result<(), InternalError> {
        sqlx::query!(
            r#"
            INSERT INTO user_feed (
                user_id, feed_id, name, icon, logo, description, created_time
            ) VALUES (?,?,?,?,?,?,?)
            "#,
            self.user_id,
            self.feed_id,
            self.name,
            self.icon,
            self.logo,
            self.description,
            self.created_time,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn retrieve_feed_by_user(
        user_id: i32,
        pool: &DatabasePool,
    ) -> Result<Vec<UserFeed>, InternalError> {
        let res = sqlx::query_as!(
            UserFeed,
            r#"
            SELECT 
                user_id,
                feed_id,
                url,
                IF (user_feed.name IS NULL, feed_profile.name, user_feed.name) as name,
                IF (user_feed.icon IS NULL, feed_profile.icon, user_feed.icon) as icon,
                IF (user_feed.logo IS NULL, feed_profile.logo, user_feed.logo) as logo,
                IF (user_feed.description IS NULL, feed_profile.description, user_feed.description) as description,
                created_time
            FROM user_feed 
            JOIN feed_profile 
            ON user_feed.feed_id = feed_profile.id
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
}
