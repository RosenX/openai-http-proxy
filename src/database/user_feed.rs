use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::common::errors::InternalError;

use super::DatabasePool;

#[derive(Clone, Debug, FromRow)]
pub struct UserFeed {
    pub user_id: i32,
    pub url: String,
    pub name: String,
    pub icon: Option<String>,
    pub created_time: DateTime<Utc>,
}

impl UserFeed {
    pub async fn retrieve_feed_by_user(
        user_id: i32,
        pool: &DatabasePool,
    ) -> Result<Vec<UserFeed>, InternalError> {
        let res = sqlx::query_as!(
            UserFeed,
            r#"
            SELECT * from user_feed
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
}
