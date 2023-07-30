#![deny(unused_crate_dependencies)]

mod feed_group_manager;
mod feed_item_manager;
mod feed_manager;
mod feed_update_record_manager;
mod service;

use abi::{
    ContentPullRequest, ContentPullResponse, ContentPushRequest, ContentPushResponse, DbService,
    InternalError, SqlValue, SubscribeFeedRequest, SubscribeFeedResponse, UserId,
};
use async_trait::async_trait;
use mockall::automock;

pub struct ContentSyncService {
    db_service: DbService,
}

#[async_trait]
pub trait Dispatcher<Req> {
    type Resp;
    async fn dispatch(&self, user_id: &UserId, request: Req) -> Result<Self::Resp, InternalError>;
}

#[async_trait]
pub trait TablePullOp {
    type Error;
    async fn pull(
        db: DbService,
        user_id: &UserId,
        last_sync_timestamp: Option<i64>,
        client_name: &str,
    ) -> Result<Vec<Self>, Self::Error>
    where
        Self: Sized;
}

#[async_trait]
pub trait TableDeleteOp {
    type Error;
    async fn delete(db: DbService, user_id: &UserId) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait TablePushOp {
    type Error;
    async fn push(
        data: Vec<Self>,
        db: DbService,
        user_id: &UserId,
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
        user_id: &UserId,
        request: ContentPullRequest,
    ) -> Result<ContentPullResponse, InternalError>;

    async fn push(
        &self,
        user_id: &UserId,
        request: ContentPushRequest,
    ) -> Result<ContentPushResponse, InternalError>;

    async fn delete(&self, user_id: &UserId) -> Result<(), InternalError>;

    // TODO merge this method to push
    async fn subscribe_feed(
        &self,
        user_id: &UserId,
        request: SubscribeFeedRequest,
    ) -> Result<SubscribeFeedResponse, InternalError>;
}

pub trait TableName {
    fn table_name() -> String;
}

pub trait InsertSqlProvider: TableName {
    fn sql_columns() -> String;
    fn sql_values(&self, user_id: &UserId, client_name: String) -> Vec<SqlValue>;
    fn sql_conflict() -> String;
}
