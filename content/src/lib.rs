mod content_manager;

use abi::DbPool;
use async_trait::async_trait;

pub struct ContentManager {
    pub pool: DbPool,
}

#[async_trait]
pub trait ContentOp {
    async fn create(&self, content: abi::Content) -> Result<abi::Content, abi::InternalError>;
}
