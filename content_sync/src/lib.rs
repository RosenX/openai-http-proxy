#![deny(unused_crate_dependencies)]

mod feed_group_manager;
mod feed_item_manager;
mod feed_manager;
mod feed_update_record_manager;
mod service;

use abi::{
    ContentPullRequest, ContentPullResponse, ContentPushRequest, ContentPushResponse, DbService,
    Id, InternalError, SqlValue, SubscribeFeedRequest, SubscribeFeedResponse, INSERT_CHUNK_SIZE,
};
use async_trait::async_trait;
use mockall::automock;

pub struct ContentSyncService {
    db_service: DbService,
}

#[async_trait]
pub trait Dispatcher<Req> {
    type Resp;
    async fn dispatch(&self, user_id: Id, request: Req) -> Result<Self::Resp, InternalError>;
}

#[async_trait]
pub trait TablePullOp {
    type Error;
    async fn pull(
        db: DbService,
        user_id: Id,
        last_sync_timestamp: Option<i64>,
        client_name: &str,
    ) -> Result<Vec<Self>, Self::Error>
    where
        Self: Sized;
}

#[async_trait]
pub trait TableDeleteOp {
    type Error;
    async fn delete(db: DbService, user_id: Id) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait TablePushOp {
    type Error;
    async fn push(
        data: Vec<Self>,
        db: DbService,
        user_id: Id,
        client_name: &str,
    ) -> Result<(), Self::Error>
    where
        Self: Sized;
}

#[async_trait]
#[automock]
pub trait ContentSyncServiceApi {
    async fn pull(
        &self,
        user_id: Id,
        request: ContentPullRequest,
    ) -> Result<ContentPullResponse, InternalError>;

    async fn push(
        &self,
        user_id: Id,
        request: ContentPushRequest,
    ) -> Result<ContentPushResponse, InternalError>;

    async fn delete(&self, user_id: Id) -> Result<(), InternalError>;

    // TODO merge this method to push
    async fn subscribe_feed(
        &self,
        user_id: Id,
        request: SubscribeFeedRequest,
    ) -> Result<SubscribeFeedResponse, InternalError>;
}

pub trait TableName {
    fn table_name() -> String;
}

pub trait InsertSqlProvider: TableName {
    fn sql_columns() -> String;
    fn sql_values(&self, user_id: Id, client_name: String) -> Vec<SqlValue>;
    fn sql_conflict() -> String;
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
