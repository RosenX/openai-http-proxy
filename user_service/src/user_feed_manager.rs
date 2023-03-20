use abi::DbPool;

use crate::UserFeedManager;

impl UserFeedManager {
    pub fn new(pool: DbPool) -> Self {
        UserFeedManager { pool }
    }
}
