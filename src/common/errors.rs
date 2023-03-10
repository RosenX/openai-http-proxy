use bcrypt::BcryptError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InternalError {
    // user 
    #[error("email is already in use, error info: {0}")]
    DuplicateEmail(String),

    #[error("token is expired, error info: {0}")]
    TokenExpired(String),

    #[error("error info: {0}")]
    JsonWebTokenError(String),

    #[error("error info: {0}")]
    InvalidAuthToken(String),

    #[error("user not exist")]
    UserNotExist,

    #[error("wrong password")]
    WrongPassword,

    // source
    #[error("rss source not exists: {0}")]
    SourceNotExist(String),

    // content
    #[error("error info: {0}")]
    InvalidUrl(String),

    // database
    #[error("error info: {0}")]
    DatabaseError(#[from] sqlx::Error),

    // encrypt
    #[error("error info: {0}")]
    EncryptError(#[from] BcryptError),

    // network
    #[error("error info: {0}")]
    NetworkError(#[from] reqwest::Error),

    // Feed parse Error
    #[error("error info: {0}")]
    FeedParseError(#[from] feed_rs::parser::ParseFeedError),
}
