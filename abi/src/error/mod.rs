use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum InternalError {
    // auth
    #[error("Invalid Token: {0}")]
    InvalidToken(String),

    #[error("Invalid User: {0}")]
    InvalidUser(String),

    #[error("Password wrong: {0}")]
    WrongPassword(String),

    // database
    #[error("Could not start transaction: {0}")]
    CouldNotStartTransaction(String),

    #[error("Database Error: {0}")]
    DatabaseStartError(String),

    #[error("Database Insert Error: {0}")]
    DatabaseInsertError(String),

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
        let (code, message) = match self {
            InternalError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            InternalError::InvalidUser(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            InternalError::WrongPassword(_) => (StatusCode::UNAUTHORIZED, self.to_string()),

            InternalError::CouldNotStartTransaction(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::DatabaseStartError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            InternalError::DatabaseInsertError(_) => {
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
