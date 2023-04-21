use abi::InternalError;
use log::error;

pub enum SuccessResponse<T> {
    #[response(status = 201)]
    Created(Json<T>),

    // #[response(status = 204)]
    // Deleted(Json<T>),
    #[response(status = 200)]
    Success(Json<T>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorInfo {
    code: Option<i32>,
    message: String,
}

impl ErrorInfo {
    pub fn new(code: Option<i32>, message: String) -> ErrorInfo {
        ErrorInfo { code, message }
    }
}

#[derive(Responder)]
pub enum ErrorResponse {
    #[response(status = 401)]
    Unauthorized(Json<ErrorInfo>),

    Default(Json<ErrorInfo>),
    // #[response(status = 400)]
    // BadRequest(Json<ErrorInfo>),
}

impl From<InternalError> for ErrorResponse {
    fn from(err: InternalError) -> Self {
        error!("{}", err);
        match err {
            InternalError::WrongPassword => {
                ErrorResponse::Unauthorized(ErrorInfo::new(None, err.to_string()).into())
            }
            _ => ErrorResponse::default(),
        }
    }
}

impl Default for ErrorResponse {
    fn default() -> Self {
        ErrorResponse::Default(ErrorInfo::new(None, "服务器错误".to_string()).into())
    }
}
