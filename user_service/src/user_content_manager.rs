use abi::DbPool;

use crate::UserContentManager;

impl UserContentManager {
    pub fn new(pool: DbPool) -> Self {
        UserContentManager { pool }
    }
}
