#![deny(unused_crate_dependencies)]

mod feed_group_manager;
mod feed_item_manager;
mod feed_manager;
mod feed_update_record_manager;
mod service;

use abi::{
    ContentPullRequest, ContentPullResponse, ContentPushRequest, ContentPushResponse, DbService,
    Id, InternalError, SubscribeFeedRequest, SubscribeFeedResponse,
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
