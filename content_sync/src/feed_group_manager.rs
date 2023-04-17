use abi::{timestamp_to_datetime, DbService, FeedGroup, Id, InternalError};
use async_trait::async_trait;

pub struct FeedGroupManager {
    db_service: DbService,
}

impl FeedGroupManager {
    pub fn new(db_service: DbService) -> Self {
        Self { db_service }
    }
}

#[async_trait]
pub trait FeedGroupManageOp {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_groups: Vec<FeedGroup>,
    ) -> Result<(), abi::InternalError>;
    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
    ) -> Result<Vec<FeedGroup>, abi::InternalError>;
}

#[async_trait]
impl FeedGroupManageOp for FeedGroupManager {
    async fn insert_batch(
        &self,
        user_id: Id,
        feed_groups: Vec<FeedGroup>,
    ) -> Result<(), InternalError> {
        let values = feed_groups
            .iter()
            .map(|feed_group| {
                format!(
                    "({}, {}, {}, {})",
                    user_id, feed_group.name, feed_group.description, feed_group.update_time
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let sql = format!(
            "INSERT INTO feed_group (user_id, group_id, name, description, update_time) VALUES {}",
            values
        );
        sqlx::query(&sql).execute(self.db_service.as_ref()).await?;
        Ok(())
    }

    async fn query_need_sync(
        &self,
        user_id: Id,
        timestamp: Option<i64>,
    ) -> Result<Vec<FeedGroup>, InternalError> {
        let result = match timestamp {
            Some(t) => {
                let sql = format!(
                    "SELECT * FROM feed_group WHERE user_id = {} AND update_time > '{}'",
                    user_id,
                    timestamp_to_datetime(t)
                );
                sqlx::query_as::<_, FeedGroup>(&sql)
                    .fetch_all(self.db_service.as_ref())
                    .await?
            }
            None => vec![],
        };
        Ok(result)
    }
}
