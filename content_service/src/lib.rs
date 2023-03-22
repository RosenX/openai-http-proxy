mod content_manager;
mod content_service;
mod feed_manager;
mod feed_parser;

use abi::{Content, DbService, FeedProfile, HttpService, InternalError, MD5Wapper, Url};
use async_trait::async_trait;
use feed_rs::model::Feed;

struct ContentManager {
    db_service: DbService,
}

struct FeedManager {
    db_service: DbService,
}

struct FeedParser {
    http_client: HttpService,
}

pub struct ContentService {
    content_manager: ContentManager,
    feed_manager: FeedManager,
    feed_parser: FeedParser,
}

#[async_trait]
pub trait ContentManageOp {
    async fn create(&self, mut content: abi::Content) -> Result<abi::Content, abi::InternalError>;
    async fn create_multiple(
        &self,
        mut content_list: Vec<abi::Content>,
    ) -> Result<Vec<abi::Content>, abi::InternalError>;
    async fn query_all_md5(&self) -> Result<Vec<MD5Wapper>, abi::InternalError>;
}

#[async_trait]
pub trait FeedManageOp {
    async fn create(
        &self,
        feed_profile: abi::FeedProfile,
    ) -> Result<abi::FeedProfile, abi::InternalError>;
    async fn query_all(&self) -> Result<Vec<abi::FeedProfile>, abi::InternalError>;
}

#[async_trait]
pub trait ContentServiceApi {
    async fn create_feed(
        &self,
        feed_request: abi::CreateFeedRequest,
    ) -> Result<abi::FeedContentResponse, abi::InternalError>;
}

#[async_trait]
pub trait FeedParserOp {
    async fn fetch_feed_from_url(&self, url: &Url) -> Result<Feed, abi::InternalError>;
    async fn fetch_feed_multiple(
        &self,
        feed_list: Vec<FeedProfile>,
    ) -> Result<Vec<Feed>, abi::InternalError>;
    async fn fetch_feed(&self, feed_profile: FeedProfile) -> Result<Feed, abi::InternalError>;
    // todo 改成异步
    fn parse_feed_multiple(&self, feed_list: Vec<Feed>) -> Result<Vec<Content>, InternalError>;
    fn parse_feed(&self, feed: Feed) -> Result<Vec<Content>, InternalError>;
}
