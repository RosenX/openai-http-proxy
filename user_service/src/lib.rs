mod service;

use abi::{
    DbService, InternalError, PurchaseVerifyResponse, UserActivityRequest, UserServiceConfig,
};
use async_trait::async_trait;

pub struct UserService {
    db: DbService,
    config: UserServiceConfig,
}

#[async_trait]
pub trait UserServiceApi {
    async fn user_activity(&self, request: UserActivityRequest) -> Result<(), InternalError>;
    async fn purchase_verify(
        &self,
        request: abi::PurchaseVerifyRequest,
    ) -> Result<PurchaseVerifyResponse, InternalError>;
}
