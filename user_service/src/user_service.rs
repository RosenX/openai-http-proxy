use abi::{DbPool, InternalError, UserInformation, UserProfile};
use async_trait::async_trait;

use crate::{
    config::ServiceConfig, UserContentManager, UserFeedManager, UserManager, UserManagerOp,
    UserService, UserServiceApi,
};

impl UserService {
    pub fn new(pool: DbPool, service_config: ServiceConfig) -> Self {
        Self {
            user_manager: UserManager::new(pool.clone()),
            user_feed_manager: UserFeedManager::new(pool.clone()),
            user_content_manager: UserContentManager::new(pool),
            config: service_config,
        }
    }
}

#[async_trait]
impl UserServiceApi for UserService {
    type Error = InternalError;

    async fn register_by_email(
        &self,
        request: abi::RegisterReq,
    ) -> Result<abi::UserProfile, Self::Error> {
        let user_info = UserInformation::try_from(request)?;
        let user_info = self.user_manager.create(user_info).await?;
        let user_profile = UserProfile::from(user_info);
        Ok(user_profile)
    }
}
