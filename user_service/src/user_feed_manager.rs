use abi::{timestamp_to_datetime, DbService, Id, InternalError, UserFeed};
use async_trait::async_trait;

use crate::{UserFeedManager, UserFeedManagerOp};

impl UserFeedManager {
    pub fn new(db_service: DbService) -> Self {
        UserFeedManager { db_service }
    }
}

#[async_trait]
impl UserFeedManagerOp for UserFeedManager {
    type Error = InternalError;
    async fn create(&self, mut user_feed: UserFeed) -> Result<UserFeed, Self::Error> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO user_feed (
                user_id, feed_id, name, logo, description, created_time
            ) VALUES ($1,$2,$3,$4,$5,$6)
            RETURNING id
            "#,
            user_feed.user_id,
            user_feed.feed_id,
            user_feed.name,
            user_feed.logo,
            user_feed.description,
            timestamp_to_datetime(user_feed.created_time),
        )
        .fetch_one(self.db_service.as_ref())
        .await?;
        user_feed.id = id;
        Ok(user_feed)
    }
    async fn query(&self, user_id: Id) -> Result<Vec<UserFeed>, Self::Error> {
        let user_feeds = sqlx::query_as("SELECT * FROM user_feed WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(self.db_service.as_ref())
            .await?;
        Ok(user_feeds)
    }
}
