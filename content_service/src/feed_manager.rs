use abi::{DbPool, FeedProfile, InternalError};
use async_trait::async_trait;

use crate::{FeedManageOp, FeedManager};

impl FeedManager {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FeedManageOp for FeedManager {
    async fn create(&self, feed_profile: abi::FeedProfile) -> Result<FeedProfile, InternalError> {
        let fp = sqlx::query_as!(
            FeedProfile,
            r#"
            INSERT INTO feed_profile
                (url, name, logo, icon, description, category_algo, tags_algo)
            VALUES ($1,$2,$3,$4,$5,$6,$7)
            RETURNING *
            "#,
            feed_profile.url,
            feed_profile.name,
            feed_profile.logo,
            feed_profile.icon,
            feed_profile.description,
            feed_profile.category_algo,
            feed_profile.tags_algo
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(fp)
    }
}
