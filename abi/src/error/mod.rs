use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum InternalError {
    // auth
    #[error("Invalid Token: {0}")]
    InvalidToken(String),

    #[error("Invalid User: {0}")]
    InvalidUser(String),

    #[error("Password wrong: {0}")]
    WrongPassword(String),

    #[error("User exist: {0}")]
    UserExist(String),

    #[error("User not exist: {0}")]
    UserNotExist(String),

    // database
    #[error("Could not start transaction: {0}")]
    CouldNotStartTransaction(String),

    #[error("Database Error: {0}")]
    DatabaseStartError(String),

    #[error("Database Insert Error: {0}")]
    DatabaseInsertError(String),

    #[error("Database Delete Error: {0}")]
    DatabaseDeleteError(String),

    #[error("Database Select Error: {0}")]
    DatabaseSelectError(String),

    // other
    #[error("Encrypt error when veriry: {0}")]
    EncryptVerifyError(String),

    #[error("Encrypt error when hash: {0}")]
    EncryptHashError(String),

    #[error("Jwt encode error: {0}")]
    JwtEncodeError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        error!("InternalError: {}", self);
        let (code, message) = match self {
            InternalError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            InternalError::InvalidUser(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            InternalError::WrongPassword(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            InternalError::UserExist(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            InternalError::UserNotExist(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            InternalError::CouldNotStartTransaction(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::DatabaseStartError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::DatabaseInsertError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::DatabaseDeleteError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::DatabaseSelectError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::EncryptHashError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::EncryptVerifyError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::JwtEncodeError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::InvalidRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (code, message).into_response()
    }
}

impl<T> From<InternalError> for crate::Response<T> {
    fn from(error: InternalError) -> Self {
        error!("InternalError: {}", error);
        match error {
            InternalError::InvalidToken(_) => {
                Self::new(StatusCode::UNAUTHORIZED.as_u16(), None, error.to_string())
            }

            InternalError::InvalidUser(_) => {
                Self::new(StatusCode::UNAUTHORIZED.as_u16(), None, error.to_string())
            }
            InternalError::WrongPassword(_) => {
                Self::new(StatusCode::UNAUTHORIZED.as_u16(), None, error.to_string())
            }
            InternalError::UserExist(_) => {
                Self::new(StatusCode::BAD_REQUEST.as_u16(), None, error.to_string())
            }
            InternalError::UserNotExist(_) => {
                Self::new(StatusCode::BAD_REQUEST.as_u16(), None, error.to_string())
            }
            InternalError::CouldNotStartTransaction(_) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                None,
                error.to_string(),
            ),
            InternalError::DatabaseStartError(_) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                None,
                error.to_string(),
            ),
            InternalError::DatabaseInsertError(_) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                None,
                error.to_string(),
            ),
            InternalError::DatabaseDeleteError(_) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                None,
                error.to_string(),
            ),
            InternalError::DatabaseSelectError(_) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                None,
                error.to_string(),
            ),
            InternalError::EncryptHashError(_) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                None,
                error.to_string(),
            ),
            InternalError::EncryptVerifyError(_) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                None,
                error.to_string(),
            ),
            InternalError::JwtEncodeError(_) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                None,
                error.to_string(),
            ),
            InternalError::InvalidRequest(_) => {
                Self::new(StatusCode::BAD_REQUEST.as_u16(), None, error.to_string())
            }
        }
    }
}
