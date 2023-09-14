mod service;

use abi::{DbService, InternalError, UserActivityRequest};
use async_trait::async_trait;

pub struct UserService {
    db_service: DbService,
}

#[async_trait]
pub trait UserServiceApi {
    async fn user_activity(&self, request: UserActivityRequest) -> Result<(), InternalError>;
}
