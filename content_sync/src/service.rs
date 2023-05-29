use std::vec;

use abi::{
    ContentPullRequest, ContentPullResponse, ContentPushRequest, ContentPushResponse, DbService,
    Feed, FeedGroup, FeedItem, FeedUpdateRecord, Id, InternalError, SqlValue, SyncTimestamp,
    INSERT_CHUNK_SIZE,
};
use async_trait::async_trait;

use crate::{
    ContentSyncService, ContentSyncServiceApi, Dispatcher, InsertSqlProvider, TableDeleteOp,
    TablePullOp, TablePushOp,
};

impl ContentSyncService {
    pub fn new(db: DbService) -> Self {
        Self { db_service: db }
    }
}

#[async_trait]
impl Dispatcher<ContentPullRequest> for ContentSyncService {
    type Resp = ContentPullResponse;
    async fn dispatch(
        &self,
        user_id: Id,
        request: ContentPullRequest,
    ) -> Result<Self::Resp, InternalError> {
        let timestamps: abi::SyncTimestamp = request.sync_timestamp;
        let client = request.client;
        let feeds_future = Feed::pull(
            self.db_service.clone(),
            user_id,
            timestamps.feed,
            &client.client_name,
        );
        let feed_items_future = FeedItem::pull(
            self.db_service.clone(),
            user_id,
            timestamps.feed_item,
            &client.client_name,
        );
        let feed_groups_future = FeedGroup::pull(
            self.db_service.clone(),
            user_id,
            timestamps.feed_group,
            &client.client_name,
        );
        let feed_update_records_future = FeedUpdateRecord::pull(
            self.db_service.clone(),
            user_id,
            timestamps.feed_update_record,
            &client.client_name,
        );

        let (mut feeds, mut feed_groups, mut feed_items, mut feed_update_records) = tokio::try_join!(
            feeds_future,
            feed_groups_future,
            feed_items_future,
            feed_update_records_future
        )?;

        feeds.sort_by(|a, b| a.sync_time.cmp(&b.sync_time));
        feed_groups.sort_by(|a, b| a.sync_time.cmp(&b.sync_time));
        feed_items.sort_by(|a, b| a.sync_time.cmp(&b.sync_time));
        feed_update_records.sort_by(|a, b| a.sync_time.cmp(&b.sync_time));

        let sync_timestamp = SyncTimestamp {
            // sync time一定有值，数据库NOT NULL
            // sort by sync time and return the latest
            feed: feeds.last().map(|f| f.sync_time.unwrap()),
            feed_group: feed_groups.last().map(|f| f.sync_time.unwrap()),
            feed_item: feed_items.last().map(|f| f.sync_time.unwrap()),
            feed_update_record: feed_update_records.last().map(|f| f.sync_time.unwrap()),
        };
        Ok(ContentPullResponse {
            client,
            feeds,
            feed_items,
            feed_groups,
            feed_update_records,
            sync_timestamp,
        })
    }
}

#[async_trait]
impl Dispatcher<ContentPushRequest> for ContentSyncService {
    type Resp = ContentPushResponse;
    async fn dispatch(
        &self,
        user_id: Id,
        request: ContentPushRequest,
    ) -> Result<Self::Resp, InternalError> {
        let ContentPushRequest {
            client,
            feeds,
            feed_groups,
            feed_items,
            feed_update_records,
        } = request;

        let feeds_future = Feed::push(feeds, self.db_service.clone(), user_id, &client.client_name);
        let feed_groups_future = FeedGroup::push(
            feed_groups,
            self.db_service.clone(),
            user_id,
            &client.client_name,
        );
        let feed_items_future = FeedItem::push(
            feed_items,
            self.db_service.clone(),
            user_id,
            &client.client_name,
        );
        let feed_update_records_future = FeedUpdateRecord::push(
            feed_update_records,
            self.db_service.clone(),
            user_id,
            &client.client_name,
        );

        tokio::try_join!(
            feeds_future,
            feed_groups_future,
            feed_items_future,
            feed_update_records_future
        )?;

        Ok(abi::ContentPushResponse {
            client,
            message: "Success".to_string(), // TODO
        })
    }
}

#[async_trait]
impl ContentSyncServiceApi for ContentSyncService {
    async fn pull(
        &self,
        user_id: i32,
        request: abi::ContentPullRequest,
    ) -> Result<abi::ContentPullResponse, InternalError> {
        self.dispatch(user_id, request).await
    }

    async fn push(
        &self,
        user_id: i32,
        request: abi::ContentPushRequest,
    ) -> Result<abi::ContentPushResponse, abi::InternalError> {
        self.dispatch(user_id, request).await
    }

    async fn delete(&self, user_id: Id) -> Result<(), abi::InternalError> {
        let feed_groups_future = FeedGroup::delete(self.db_service.clone(), user_id);
        let feeds_future = Feed::delete(self.db_service.clone(), user_id);
        let feed_items_future = FeedItem::delete(self.db_service.clone(), user_id);
        let feed_update_records_future = FeedUpdateRecord::delete(self.db_service.clone(), user_id);

        tokio::try_join!(
            feed_groups_future,
            feeds_future,
            feed_items_future,
            feed_update_records_future
        )?;

        Ok(())
    }

    async fn subscribe_feed(
        &self,
        user_id: Id,
        request: abi::SubscribeFeedRequest,
    ) -> Result<abi::SubscribeFeedResponse, abi::InternalError> {
        let abi::SubscribeFeedRequest { client, feed_info } = request;

        let feed: Feed = feed_info.into();
        Feed::push(
            vec![feed],
            self.db_service.clone(),
            user_id,
            &client.client_name,
        )
        .await?;

        Ok(abi::SubscribeFeedResponse {
            client,
            message: "Success".to_string(), // TODO
        })
    }
}

pub fn generate_insert_query<T: InsertSqlProvider>(
    data: &[T],
    user_id: Id,
    client_name: &str,
) -> (String, Vec<SqlValue>) {
    let columns = T::sql_columns();
    let mut insert_query = format!("INSERT INTO {} ({}) VALUES ", T::table_name(), columns);
    let mut bindings: Vec<SqlValue> = Vec::new();

    for (i, item) in data.iter().enumerate() {
        let values = item.sql_values(user_id, client_name.to_owned());
        insert_query.push('(');
        for (j, value) in values.iter().enumerate() {
            insert_query.push_str(&format!("${},", i * values.len() + j + 1));
            bindings.push(value.to_owned());
        }
        insert_query.pop(); // 移除最后一个逗号
        insert_query.push_str("),");
    }
    insert_query.pop(); // 移除最后一个逗号
    insert_query.push_str(T::sql_conflict().as_str());
    // info!("insert binding: {}", bindings);

    (insert_query, bindings)
}

pub async fn execute_bulk_insert<T: InsertSqlProvider>(
    database: &DbService,
    data: Vec<T>,
    user_id: Id,
    client_name: &str,
) -> Result<(), InternalError> {
    // 开启事务
    let mut tx = database
        .begin()
        .await
        .map_err(|e| InternalError::CouldNotStartTransaction(e.to_string()))?;

    for chunk in data.chunks(INSERT_CHUNK_SIZE) {
        let (insert_query, bindings) = generate_insert_query(chunk, user_id, client_name);
        let mut query = sqlx::query(&insert_query);

        for binding in bindings {
            query = binding.bind(query);
        }

        query
            .execute(&mut tx)
            .await
            .map_err(|e| InternalError::DatabaseInsertError(e.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|e| InternalError::CouldNotStartTransaction(e.to_string()))?;
    Ok(())
}
