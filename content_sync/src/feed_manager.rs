use abi::{timestamp_to_datetime, DbService, Feed, Id, InternalError};
use async_trait::async_trait;

pub struct FeedManager {
    db_service: DbService,
}

impl FeedManager {
    pub fn new(db_service: DbService) -> Self {
        Self { db_service }
    }
}

#[async_trait]
pub trait FeedManageOp {
    async fn insert_batch(&self, user_id: Id, feeds: Vec<Feed>) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
    ) -> Result<Vec<Feed>, abi::InternalError>;
}

#[async_trait]
impl FeedManageOp for FeedManager {
    async fn insert_batch(&self, user_id: Id, feeds: Vec<Feed>) -> Result<(), InternalError> {
        let values = feeds
            .iter()
            .map(|feed| {
                format!(
                    "({{ {} }},{},{},{},{},{},{},{},{},{},{},{})",
                    feed.tags.join(","),
                    user_id,
                    feed.url,
                    feed.name.to_owned().unwrap_or("NULL".to_string()),
                    feed.custom_name.to_owned().unwrap_or("NULL".to_string()),
                    feed.logo.to_owned().unwrap_or("NULL".to_string()),
                    feed.custom_logo.to_owned().unwrap_or("NULL".to_string()),
                    feed.description.to_owned().unwrap_or("NULL".to_string()),
                    feed.custom_description
                        .to_owned()
                        .unwrap_or("NULL".to_string()),
                    match feed.group_id {
                        Some(id) => id.to_string(),
                        None => "NULL".to_string(),
                    },
                    feed.create_time,
                    feed.feed_type
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let sql = format!("INSERT INTO feed (user_id, feed_id, url, name, custom_name, logo, custom_logo, description, custom_description, group_id, tags, create_time, type) VALUES {}", values);
        sqlx::query(&sql).execute(self.db_service.as_ref()).await?;
        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
    ) -> Result<Vec<Feed>, InternalError> {
        let result = match timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed WHERE user_id = {} AND update_time > '{}'",
                    user_id,
                    timestamp_to_datetime(t)
                );
                sqlx::query_as::<_, Feed>(&sql)
                    .fetch_all(self.db_service.as_ref())
                    .await?
            }
            None => vec![],
        };
        Ok(result)
    }
}
