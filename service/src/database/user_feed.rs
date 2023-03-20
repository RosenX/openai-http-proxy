use abi::{DbPool, InternalError};
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use sqlx::FromRow;

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
    // pub fn new(feed_profile: FeedProfile, user: AuthorizedUser) -> Self {
    //     let now_datetime = Utc::now();
    //     Self {
    //         user_id: user.id,
    //         feed_id: feed_profile.id,
    //         url: feed_profile.url,
    //         name: Some(feed_profile.name),
    //         icon: feed_profile.icon,
    //         logo: Some(feed_profile.logo),
    //         description: feed_profile.description,
    //         created_time: now_datetime,
    //     }
    // }

    // pub async fn insert(&self, pool: &DbPool) -> Result<(), InternalError> {
    //     sqlx::query!(
    //         r#"
    //         INSERT INTO user_custom_feed (
    //             user_id, feed_id, name, icon, logo, description, created_time
    //         ) VALUES (?,?,?,?,?,?,?)
    //         "#,
    //         self.user_id,
    //         self.feed_id,
    //         self.name,
    //         self.icon,
    //         self.logo,
    //         self.description,
    //         self.created_time,
    //     )
    //     .execute(pool)
    //     .await?;
    //     Ok(())
    // }

    pub async fn retrieve_feed_by_user(
        user_id: i32,
        pool: &DbPool,
    ) -> Result<Vec<UserFeed>, InternalError> {
        let res = sqlx::query_as!(
            UserFeed,
            r#"
            SELECT
                user_id,
                feed_id,
                url,
                IF (uf.name IS NULL, fp.name, uf.name) as name,
                IF (uf.icon IS NULL, fp.icon, uf.icon) as icon,
                IF (uf.logo IS NULL, fp.logo, uf.logo) as logo,
                IF (uf.description IS NULL, fp.description, uf.description) as description,
                created_time
            FROM user_custom_feed uf
            JOIN feed_profile  fp
            ON uf.feed_id = fp.id
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
}
