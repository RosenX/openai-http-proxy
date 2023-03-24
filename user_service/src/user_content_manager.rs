use abi::{DbService, Id, InternalError, UserContent};
use async_trait::async_trait;

use crate::{UserContentManager, UserContentManagerOp};

impl UserContentManager {
    pub fn new(db_service: DbService) -> Self {
        UserContentManager { db_service }
    }
}

#[async_trait]
impl UserContentManagerOp for UserContentManager {
    type Error = InternalError;
    async fn create(&self, mut user_content: UserContent) -> Result<UserContent, Self::Error> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO user_content (
                content_id, user_id, tags, category, notes, stage
            ) VALUES ($1,$2,$3,$4,$5,$6)
            RETURNING id
            "#,
            user_content.content_id,
            user_content.user_id,
            user_content.tags,
            user_content.category,
            user_content.notes,
            user_content.stage as i16 //todo
        )
        .fetch_one(self.db_service.as_ref())
        .await?;
        user_content.id = id;
        Ok(user_content)
    }

    async fn query_latest(
        &self,
        user_id: Id,
        content_id: Id,
        size: i32,
    ) -> Result<Vec<UserContent>, Self::Error> {
        let user_content = sqlx::query_as(
            r#"
            SELECT * FROM user_content
            WHERE user_id = $1 AND content_id > $2
            ORDER BY content_id DESC
            LIMIT $3
            "#,
        )
        .bind(user_id)
        .bind(content_id)
        .bind(size)
        .fetch_all(self.db_service.as_ref())
        .await?;
        Ok(user_content)
    }

    async fn query_old(
        &self,
        user_id: Id,
        content_id: Id,
        size: i32,
    ) -> Result<Vec<UserContent>, Self::Error> {
        let user_content = sqlx::query_as(
            r#"
            SELECT * FROM user_content
            WHERE user_id = $1 AND content_id < $2
            ORDER BY content_id DESC
            LIMIT $3
            "#,
        )
        .bind(user_id)
        .bind(content_id)
        .bind(size)
        .fetch_all(self.db_service.as_ref())
        .await?;
        Ok(user_content)
    }
}
