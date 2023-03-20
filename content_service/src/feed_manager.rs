use abi::DbPool;
use async_trait::async_trait;

use crate::{FeedManageOp, FeedManager};

impl FeedManager {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FeedManageOp for FeedManager {
    async fn create(
        &self,
        mut feed_profile: abi::FeedProfile,
    ) -> Result<abi::FeedProfile, abi::InternalError> {
        let feed_id = sqlx::query!(
            r#"
            INSERT INTO feed_profile
                (url, name, logo, icon, description, category, tags)
            VALUES (?,?,?,?,?,?,?)
            "#,
            feed_profile.url,
            feed_profile.name,
            feed_profile.logo,
            feed_profile.icon,
            feed_profile.description,
            feed_profile.category,
            feed_profile.tags
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();
        feed_profile.id = feed_id as i32;
        Ok(feed_profile)
    }
}
