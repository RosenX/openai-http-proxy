use rocket::{serde::{Serialize, json::Json}, Responder};

#[derive(Responder)]
pub enum SuccessResponse<T> {
    #[response(status = 200)]
    Created(Json<T>),

    #[response(status = 200)]
    Accepted(Json<T>),

    #[response(status = 200)]
    Success(Json<T>),
}

pub const DefaultSuccessResponse: SuccessResponse<String> = 
    SuccessResponse::Success("Success".to_string().into());


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
    #[response(status =401)]
    LoginFail(Json<ErrorInfo>),

    #[response(status = 500)]
    Default(Json<ErrorInfo>)
}

pub const DefaultErrorResponse: ErrorResponse = 
    ErrorResponse::Default(ErrorInfo::new(None, "失败".to_string()).into());


// 用户登录
pub const HashError: ErrorResponse = 
    ErrorResponse::LoginFail(ErrorInfo::new(None, "密码哈希失败".to_string()).into());

pub const InvalidEmail: ErrorResponse = 
    ErrorResponse::LoginFail(ErrorInfo::new(None, "邮箱已注册".to_string()).into());

pub const UserNotExist: ErrorResponse = 
    ErrorResponse::LoginFail(ErrorInfo::new(None, "用户不存在".to_string()).into());

pub const InvalidPassword: ErrorResponse = 
    ErrorResponse::LoginFail(ErrorInfo::new(None, "密码错误".to_string()).into());

pub const InvalidRefreshToken: ErrorResponse = 
    ErrorResponse::LoginFail(ErrorInfo::new(None, "密码过期，请重新登陆".to_string()).into());

pub const JwtEncodeFail: ErrorResponse = 
    ErrorResponse::LoginFail(ErrorInfo::new(None, "登录失败".to_string()).into());
