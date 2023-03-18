use async_trait::async_trait;

#[async_trait]
pub trait ContentOp {
    async fn create(&self, content: abi::Content) -> Result<abi::Content, abi::InternalError>;
}
