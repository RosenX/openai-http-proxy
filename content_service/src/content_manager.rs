use abi::DbPool;
use async_trait::async_trait;

use crate::{ContentManageOp, ContentManager};

impl ContentManager {
    pub fn new(pool: DbPool) -> Self {
        ContentManager { pool }
    }
}

#[async_trait]
impl ContentManageOp for ContentManager {
    async fn create(&self, mut content: abi::Content) -> Result<abi::Content, abi::InternalError> {
        let id = sqlx::query!(
            r#"
            INSERT INTO feed_post (
                feed_id,
                title,
                publish_time,
                cover,
                authors,
                link,
                content,
                summary,
                summary_algo,
                category_algo,
                tags_algo
            )
            VALUES (?,?,?,?,?,?,?,?,?,?,?)
            "#,
            content.feed_id,
            content.title,
            content.publish_time,
            content.cover,
            content.authors,
            content.link,
            content.content,
            content.summary,
            content.summary_algo,
            content.category_algo,
            content.tags_algo
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();
        content.id = id as i32;
        Ok(content)
    }

    // todo rewrite using tokio
    async fn create_multiple(
        &self,
        content_list: Vec<abi::Content>,
    ) -> Result<Vec<abi::Content>, abi::InternalError> {
        // let stream = stream::iter(0..content_list.len());
        let mut return_content_list = Vec::with_capacity(content_list.len());
        for content in content_list {
            let content = self.create(content).await?;
            return_content_list.push(content);
        }
        Ok(return_content_list)
    }
}
