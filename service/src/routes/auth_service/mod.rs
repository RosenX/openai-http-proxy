mod authorization;

use abi::{Token, UserProfile};

use authorization::AuthConfig;

pub struct AuthorizedUser {
    user_profile: UserProfile,
}

pub struct AuthService {
    pub config: AuthConfig,
}

pub trait Authurize {
    type Error;
    fn authurize_user(&self, token: Token) -> Result<AuthorizedUser, Self::Error>;
}
