use abi::{DbService, Feed, Id, InternalError};
use async_trait::async_trait;

use crate::{
    feed_group_manager::{FeedGroupManageOp, FeedGroupManager},
    feed_item_manager::{FeedItemManageOp, FeedItemManager},
    feed_manager::{FeedManageOp, FeedManager},
    feed_update_record_manager::{FeedUpdateRecordManageOp, FeedUpdateRecordManager},
    ContentSyncService, ContentSyncServiceApi,
};

impl ContentSyncService {
    pub fn new(db: DbService) -> Self {
        Self {
            feed_manager: FeedManager::new(db.clone()),
            feed_group_manager: FeedGroupManager::new(db.clone()),
            feed_item_manager: FeedItemManager::new(db.clone()),
            feed_update_record_manager: FeedUpdateRecordManager::new(db),
        }
    }
}

#[async_trait]
impl ContentSyncServiceApi for ContentSyncService {
    async fn pull(
        &self,
        user_id: i32,
        request: abi::ContentPullRequest,
    ) -> Result<abi::ContentPullResponse, InternalError> {
        let timestamps = request.sync_timestamp;
        let client = request.client;

        let feeds =
            self.feed_manager
                .query_need_sync(user_id, timestamps.feed, client.client_name.clone());
        let feed_groups = self.feed_group_manager.query_need_sync(
            user_id,
            timestamps.feed_group,
            client.client_name.clone(),
        );
        let feed_items = self.feed_item_manager.query_need_sync(
            user_id,
            timestamps.feed_item,
            client.client_name.clone(),
        );
        let feed_update_records = self.feed_update_record_manager.query_need_sync(
            user_id,
            timestamps.feed_update_record,
            client.client_name.clone(),
        );

        let (mut feeds, mut feed_groups, mut feed_items, mut feed_update_records) =
            tokio::try_join!(feeds, feed_groups, feed_items, feed_update_records)?;

        feeds.sort_by(|a, b| b.update_time.cmp(&a.update_time));
        feed_groups.sort_by(|a, b| b.update_time.cmp(&a.update_time));
        feed_items.sort_by(|a, b| b.update_time.cmp(&a.update_time));
        feed_update_records.sort_by(|a, b| b.update_time.cmp(&a.update_time));

        let sync_timestamp = abi::SyncTimestamp {
            feed_group: feed_groups.last().map(|feed_group| feed_group.update_time),
            feed: feeds.last().map(|feed| feed.update_time),
            feed_item: feed_items.last().map(|feed_item| feed_item.update_time),
            feed_update_record: feed_update_records
                .last()
                .map(|feed_update_record| feed_update_record.update_time),
        };

        Ok(abi::ContentPullResponse {
            client,
            sync_timestamp,
            feeds,
            feed_groups,
            feed_items,
            feed_update_records,
        })
    }

    async fn push(
        &self,
        user_id: i32,
        request: abi::ContentPushRequest,
    ) -> Result<abi::ContentPushResponse, abi::InternalError> {
        let abi::ContentPushRequest {
            client,
            feeds,
            feed_groups,
            feed_items,
            feed_update_records,
        } = request;

        let feeds = self
            .feed_manager
            .insert_batch(user_id, feeds, client.client_name.clone());
        let feed_groups =
            self.feed_group_manager
                .insert_batch(user_id, feed_groups, client.client_name.clone());
        let feed_items =
            self.feed_item_manager
                .insert_batch(user_id, feed_items, client.client_name.clone());
        let feed_update_records = self.feed_update_record_manager.insert_batch(
            user_id,
            feed_update_records,
            client.client_name.clone(),
        );

        tokio::try_join!(feeds, feed_groups, feed_items, feed_update_records)?;

        Ok(abi::ContentPushResponse {
            client,
            message: "Success".to_string(), // TODO
        })
    }

    async fn delete_user_content(&self, user_id: Id) -> Result<(), abi::InternalError> {
        let feed_groups = self.feed_group_manager.delete_by_user_id(user_id);
        let feeds = self.feed_manager.delete_by_user_id(user_id);
        let feed_items = self.feed_item_manager.delete_by_user_id(user_id);
        let feed_update_records = self.feed_update_record_manager.delete_by_user_id(user_id);

        tokio::try_join!(feed_groups, feeds, feed_items, feed_update_records)?;

        Ok(())
    }

    async fn subscribe_feed(
        &self,
        user_id: Id,
        request: abi::SubscribeFeedRequest,
    ) -> Result<abi::SubscribeFeedResponse, abi::InternalError> {
        let abi::SubscribeFeedRequest { client, feed_info } = request;

        let feed: Feed = feed_info.into();
        self.feed_manager
            .insert(user_id, feed, client.client_name.clone())
            .await?;

        Ok(abi::SubscribeFeedResponse {
            client,
            message: "Success".to_string(), // TODO
        })
    }
}
