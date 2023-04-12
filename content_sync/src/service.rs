use abi::DbService;

use crate::{
    feed_group_manager::FeedGroupManager, feed_item_manager::FeedItemManager,
    feed_manager::FeedManager, feed_update_record_manager::FeedUpdateRecordManager,
    ContentSyncService,
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
