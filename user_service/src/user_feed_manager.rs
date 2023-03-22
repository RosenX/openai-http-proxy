use abi::{DbPool, InternalError, UserFeed, UserId};
use async_trait::async_trait;

use crate::{UserFeedManager, UserFeedManagerOp};

impl UserFeedManager {
    pub fn new(pool: DbPool) -> Self {
        UserFeedManager { pool }
    }
}

#[async_trait]
impl UserFeedManagerOp for UserFeedManager {
    type Error = InternalError;
    async fn create(&self, user_feed: UserFeed) -> Result<UserFeed, Self::Error> {
        let uf = sqlx::query_as!(
            UserFeed,
            r#"
            INSERT INTO user_feed (
                user_id, feed_id, name, logo, description, created_time
            ) VALUES ($1,$2,$3,$4,$5,$6)
            RETURNING *
            "#,
            user_feed.user_id,
            user_feed.feed_id,
            user_feed.name,
            user_feed.logo,
            user_feed.description,
            user_feed.created_time,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(uf)
    }
    async fn query(&self, user_id: UserId) -> Result<Vec<UserFeed>, Self::Error> {
        let user_feeds = sqlx::query_as!(
            UserFeed,
            r#"
            SELECT * FROM user_feed WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(user_feeds)
    }
}
