use std::string::FromUtf8Error;

use bcrypt::BcryptError;

#[derive(Debug, thiserror::Error)]
pub enum InternalError {
    // user
    // #[error("email is already in use, error info: {0}")]
    // DuplicateEmail(String),
    #[error("token is expired, error info: {0}")]
    TokenExpired(String),

    #[error("error info: {0}")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),

    #[error("error info: {0}")]
    InvalidAuthToken(String),

    #[error("user not exist")]
    UserNotExist,

    #[error("wrong password")]
    WrongPassword,

    // source
    // #[error("rss source not exists: {0}")]
    // SourceNotExist(String),

    // content
    // #[error("error info: {0}")]
    // InvalidUrl(String),

    // database
    #[error("error info: {0}")]
    DatabaseError(#[from] sqlx::Error),

    // encrypt
    #[error("encrypt error {0}")]
    EncryptError(String),

    // network
    #[error("error info: {0}")]
    NetworkError(#[from] reqwest::Error),

    // Feed parse Error
    #[error("error info: {0}")]
    FeedParseError(#[from] feed_rs::parser::ParseFeedError),

    #[error("error info: md5 error")]
    MD5Error(#[from] FromUtf8Error),
}

impl From<BcryptError> for InternalError {
    fn from(value: BcryptError) -> Self {
        Self::EncryptError(value.to_string())
    }
}
