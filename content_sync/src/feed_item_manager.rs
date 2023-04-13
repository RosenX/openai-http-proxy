use abi::{DbService, FeedItem, Id, InternalError};
use async_trait::async_trait;

pub struct FeedItemManager {
    db_service: DbService,
}

impl FeedItemManager {
    pub fn new(db_service: DbService) -> Self {
        Self { db_service }
    }
}

#[async_trait]
pub trait FeedItemManageOp {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_items: Vec<FeedItem>,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: i64,
    ) -> Result<Vec<FeedItem>, abi::InternalError>;
}

#[async_trait]
impl FeedItemManageOp for FeedItemManager {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_items: Vec<FeedItem>,
    ) -> Result<(), InternalError> {
        // TODO 有没有更好的方法？
        let values = feed_items
            .into_iter()
            .map(|feed_item| {
                format!(
                    "({{ {} }}, {}, {}, {}, {}, {},{}, {}, {}, {}, {}, {},{}, {}, {}, {}, {}, {})",
                    feed_item.tags.join(","),
                    user_id,
                    feed_item.id,
                    feed_item.is_focus,
                    feed_item.is_seen,
                    feed_item.title,
                    feed_item.cover,
                    feed_item.link,
                    feed_item.publish_time,
                    feed_item.authors,
                    feed_item.category,
                    feed_item.description,
                    feed_item.summary_algo,
                    feed_item.content,
                    feed_item.content_have_parsed,
                    feed_item.create_time,
                    feed_item.md5_string,
                    feed_item.feed_id
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let sql = format!( "INSERT INTO feed_item (tags, user_id, item_id, is_focus, is_seen, title, cover, link, publish_time, authors, category, description, summary_algo, content, content_have_parsed, create_time, md5_string, feed_id) VALUES {}", values);
        sqlx::query(&sql).execute(self.db_service.as_ref()).await?;
        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: i64,
    ) -> Result<Vec<FeedItem>, InternalError> {
        let sql = format!(
            "SELECT * FROM feed_item WHERE user_id = {} AND update_time > {}",
            user_id, timestamp
        );
        let feed_items = sqlx::query_as::<_, FeedItem>(&sql)
            .fetch_all(self.db_service.as_ref())
            .await?;
        Ok(feed_items)
    }
}
