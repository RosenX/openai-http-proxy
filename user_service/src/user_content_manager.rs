use abi::{ContentId, DbPool, InternalError, UserId, UserPost};
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
    async fn create(&self, user_content: UserPost) -> Result<UserPost, Self::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_custom_post (
                post_id, user_id, tags, category, notes
            ) VALUES (?,?,?,?,?)
            "#,
            user_content.post_id,
            user_content.user_id,
            user_content.tags,
            user_content.category,
            user_content.notes,
        )
        .execute(&self.pool)
        .await?;
        Ok(user_content)
    }

    async fn query_latest(
        &self,
        user_id: UserId,
        content_id: ContentId,
    ) -> Result<Vec<UserPost>, Self::Error> {
        let user_content = sqlx::query_as!(
            UserPost,
            r#"
            SELECT * FROM user_custom_post WHERE user_id = ? AND post_id > ?
            "#,
            user_id,
            content_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(user_content)
    }
}
