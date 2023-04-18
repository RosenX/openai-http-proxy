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
    let response = auth_service.register_by_email(request.into_inner()).await?;
    Ok(SuccessResponse::Created(Json(response)))
}

#[post("/login", data = "<request>")]
async fn login_by_email(
    request: Json<LoginRequest>,
    auth_service: &State<AuthService>,
) -> Result<SuccessResponse<AuthResponse>, ErrorResponse> {
    let response = auth_service.login_by_email(request.into_inner()).await?;
    Ok(SuccessResponse::Success(Json(response)))
}

#[post("/refresh_token", data = "<request>")]
fn refresh_token(
    request: Json<RefreshTokenRequest>,
    auth_service: &State<AuthService>,
) -> Result<SuccessResponse<AuthResponse>, ErrorResponse> {
    let response = auth_service.refresh_token(request.into_inner())?;
    Ok(SuccessResponse::Created(Json(response)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About User", |rocket| async {
        rocket.mount(
            "/user",
            routes![register_by_email, login_by_email, refresh_token,],
        )
    })
}
