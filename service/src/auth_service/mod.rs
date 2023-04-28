mod authorization;
mod user_manager;

use abi::{AuthResponse, Id, LoginRequest, RefreshTokenRequest, RegisterRequest, UserProfile};

use async_trait::async_trait;
use user_manager::UserManager;

pub use authorization::AuthServiceConfig;

pub struct AuthorizedUser {
    user_profile: UserProfile,
}

impl AuthorizedUser {
    pub fn get_user_id(&self) -> Id {
        self.user_profile.user_id
    }
}

impl From<AuthorizedUser> for UserProfile {
    fn from(val: AuthorizedUser) -> UserProfile {
        val.user_profile
    }
}

#[derive(Clone)]
pub struct AuthService {
    config: AuthServiceConfig,
    user_manager: UserManager,
}

#[async_trait]
pub trait AuthServiceApi {
    type Error;
    async fn register_by_email(
        &self,
        request: RegisterRequest,
    ) -> Result<AuthResponse, Self::Error>;
    async fn login_by_email(&self, request: LoginRequest) -> Result<AuthResponse, Self::Error>;
    fn refresh_token(
        &self,
        refresh_token: RefreshTokenRequest,
    ) -> Result<AuthResponse, Self::Error>;
    async fn delete_user_account(&self, user_id: Id) -> Result<(), Self::Error>;
}
