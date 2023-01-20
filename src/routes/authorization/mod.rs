mod jwt;
mod authorize_user;

pub use jwt::{JsonWebTokenConfig, JsonWebTokenTool, JwtToken};
pub use authorize_user::{AuthorizedUser, AuthorizedProUser, PublicData};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    token: String,
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        token.token
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Token{token: s}
    }
}

pub trait Encode {
    fn encode(self);
}

pub trait  Decode {
    fn decode(self);
}