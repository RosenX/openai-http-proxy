use abi::{timestamp_to_datetime, DbService, FeedItem, Id, InternalError, OptionDisplay};
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
        timestamp: Option<i64>,
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
                    "({{ {} }}, {}, {}, {}, {},{}, {}, {}, {},{}, {}, {}, {}, {})",
                    feed_item.tags.display(),
                    user_id,
                    feed_item.is_focus,
                    feed_item.is_seen,
                    feed_item.title.display(),
                    feed_item.cover.display(),
                    feed_item.link.display(),
                    feed_item.publish_time.display(),
                    feed_item.authors.display(),
                    feed_item.category.display(),
                    feed_item.description.display(),
                    feed_item.summary_algo.display(),
                    feed_item.create_time,
                    feed_item.md5_string,
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
        timestamp: Option<i64>,
    ) -> Result<Vec<FeedItem>, InternalError> {
        let result = match timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_item WHERE user_id = {} AND update_time > '{}'",
                    user_id,
                    timestamp_to_datetime(t)
                );
                sqlx::query_as::<_, FeedItem>(&sql)
                    .fetch_all(self.db_service.as_ref())
                    .await?
            }
            None => vec![],
        };
        Ok(result)
    }
}
