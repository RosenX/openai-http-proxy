use crate::auth_service::{AuthService, AuthServiceApi};
use crate::common::responder::{ErrorResponse, SuccessResponse};
use abi::{AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest};

use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{post, routes, State};

#[post("/register", data = "<request>")]
async fn register_by_email(
    request: Json<RegisterRequest>,
    auth_service: &State<AuthService>,
) -> Result<SuccessResponse<AuthResponse>, ErrorResponse> {
    println!("register_info: {:?}", request);
    let register_info = match request.into_inner().register_info {
        Some(info) => info,
        None => return Err(ErrorResponse::default()),
    };
    let response = auth_service.register_by_email(register_info).await?;
    Ok(SuccessResponse::Created(Json(response.into())))
}

#[post("/login", data = "<request>")]
async fn login_by_email(
    request: Json<LoginRequest>,
    auth_service: &State<AuthService>,
) -> Result<SuccessResponse<AuthResponse>, ErrorResponse> {
    let login_info = match request.into_inner().login_info {
        Some(info) => info,
        None => return Err(ErrorResponse::default()),
    };
    let response = auth_service.login_by_email(login_info).await?;
    Ok(SuccessResponse::Success(Json(response.into())))
}

#[post("/refresh_token", data = "<request>")]
fn refresh_token(
    request: Json<RefreshTokenRequest>,
    auth_service: &State<AuthService>,
) -> Result<SuccessResponse<AuthResponse>, ErrorResponse> {
    let refresh_token = match request.into_inner().refresh_token {
        Some(token) => token,
        None => return Err(ErrorResponse::default()),
    };
    let response = auth_service.refresh_token(refresh_token.token)?;
    Ok(SuccessResponse::Created(Json(response.into())))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About User", |rocket| async {
        rocket.mount(
            "/user",
            routes![register_by_email, login_by_email, refresh_token,],
        )
    })
}
