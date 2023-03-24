mod authorization;
mod user_manager;

use abi::{DbService, Email, LoginInfo, RegisterInfo, Token, Tokens, UserInformation, UserProfile};

use authorization::AuthConfig;
use rocket::async_trait;

pub struct AuthorizedUser {
    user_profile: UserProfile,
}

impl From<AuthorizedUser> for UserProfile {
    fn from(val: AuthorizedUser) -> UserProfile {
        val.user_profile
    }
}

struct UserManager {
    db_service: DbService,
}

#[async_trait]
pub trait UserManagerOp {
    type Error;
    async fn create(&self, user_profile: UserInformation) -> Result<UserInformation, Self::Error>;
    async fn find_user_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<UserInformation>, Self::Error>;
}

pub struct AuthService {
    config: AuthConfig,
    user_manager: UserManager,
}

#[async_trait]
pub trait AuthServiceApi {
    type Error;
    fn authurize(&self, token: Token) -> Result<AuthorizedUser, Self::Error>;
    async fn register_by_email(&self, request: RegisterInfo) -> Result<Tokens, Self::Error>;
    async fn login_by_email(&self, request: LoginInfo) -> Result<Tokens, Self::Error>;
    fn refresh_token(&self, refresh_token: Token) -> Result<Tokens, Self::Error>;
}
