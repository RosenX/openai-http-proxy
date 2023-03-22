use abi::{ContentId, DbPool, InternalError, UserContent, UserId};
use async_trait::async_trait;

use crate::{UserContentManager, UserContentManagerOp};

impl UserContentManager {
    pub fn new(pool: DbPool) -> Self {
        UserContentManager { pool }
    }
}

#[async_trait]
impl UserContentManagerOp for UserContentManager {
    type Error = InternalError;
    async fn create(&self, user_content: UserContent) -> Result<UserContent, Self::Error> {
        let uc = sqlx::query_as!(
            UserContent,
            r#"
            INSERT INTO user_content (
                content_id, user_id, tags, category, notes, stage
            ) VALUES ($1,$2,$3,$4,$5,$6)
            RETURNING *
            "#,
            user_content.content_id,
            user_content.user_id,
            user_content.tags,
            user_content.category,
            user_content.notes,
            user_content.stage
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(uc)
    }

    async fn query_latest(
        &self,
        user_id: UserId,
        content_id: ContentId,
    ) -> Result<Vec<UserContent>, Self::Error> {
        let user_content =
            sqlx::query_as("SELECT * FROM user_content WHERE user_id = $1 AND content_id > $2")
                .bind(user_id)
                .bind(content_id)
                .fetch_all(&self.pool)
                .await?;
        Ok(user_content)
    }
}
