use abi::{Content, DbService, MD5Wapper};
use async_trait::async_trait;

use crate::{ContentManageOp, ContentManager};

impl ContentManager {
    pub fn new(db_service: DbService) -> Self {
        ContentManager { db_service }
    }
}

#[async_trait]
impl ContentManageOp for ContentManager {
    async fn create(&self, content: abi::Content) -> Result<Content, abi::InternalError> {
        let content = sqlx::query_as!(
            Content,
            r#"
            INSERT INTO content (
                feed_id,
                title,
                publish_time,
                create_time,
                cover,
                authors,
                link,
                content,
                summary,
                summary_algo,
                category_algo,
                tags_algo,
                md5
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
            RETURNING *
            "#,
            content.feed_id,
            content.title,
            content.publish_time,
            content.create_time,
            content.cover,
            content.authors,
            content.link,
            content.content,
            content.summary,
            content.summary_algo,
            content.category_algo,
            content.tags_algo,
            content.md5
        )
        .fetch_one(self.db_service.as_ref())
        .await?;
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

    async fn query_all_md5(&self) -> Result<Vec<MD5Wapper>, abi::InternalError> {
        let content = sqlx::query_as!(MD5Wapper, "SELECT md5 FROM content")
            .fetch_all(self.db_service.as_ref())
            .await?;
        Ok(content)
    }
}
