use rocket::{
    serde::{json::Json, Serialize},
    Responder,
};

#[derive(Responder)]
pub enum SuccessResponse<T> {
    #[response(status = 200)]
    Created(Json<T>),

    #[response(status = 200)]
    Accepted(Json<T>),

    #[response(status = 200)]
    Success(Json<T>),
}

impl SuccessResponse<String> {
    pub fn default_success_response() -> SuccessResponse<String> {
        SuccessResponse::Success("Success".to_string().into())
    }
}

////////////////////////////////

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorInfo {
    pub code: Option<i32>,
    pub message: String,
}

impl ErrorInfo {
    pub fn new(code: Option<i32>, message: String) -> ErrorInfo {
        ErrorInfo { code, message }
    }
}

#[derive(Responder)]
pub enum ErrorResponse {
    #[response(status = 401)]
    LoginFail(Json<ErrorInfo>),

    #[response(status = 500)]
    Default(Json<ErrorInfo>),
}

impl From<anyhow::Error> for ErrorResponse {
    fn from(err: anyhow::Error) -> Self {
        ErrorResponse::Default(ErrorInfo {
            code: None,
            message: err.to_string(),
        }.into())
    }
}

impl ErrorResponse {
    pub fn default_error_response() -> ErrorResponse {
        ErrorResponse::Default(
            ErrorInfo {
                code: None,
                message: "失败".to_string(),
            }
            .into(),
        )
    }
    // 用户登录
    pub fn hash_error() -> ErrorResponse {
        ErrorResponse::LoginFail(ErrorInfo::new(None, "密码哈希失败".to_string()).into())
    }
    pub fn invalid_email() -> ErrorResponse {
        ErrorResponse::LoginFail(ErrorInfo::new(None, "邮箱已注册".to_string()).into())
    }
    pub fn user_not_exist() -> ErrorResponse {
        ErrorResponse::LoginFail(ErrorInfo::new(None, "用户不存在".to_string()).into())
    }
    pub fn invalid_password() -> ErrorResponse {
        ErrorResponse::LoginFail(ErrorInfo::new(None, "密码错误".to_string()).into())
    }
    pub fn invalid_refresh_token() -> ErrorResponse {
        ErrorResponse::LoginFail(ErrorInfo::new(None, "密码过期，请重新登陆".to_string()).into())
    }
    pub fn jwt_encode_fail() -> ErrorResponse {
        ErrorResponse::LoginFail(ErrorInfo::new(None, "登录失败".to_string()).into())
    }
}
