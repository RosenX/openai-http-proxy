use crate::auth_service::{AuthService, AuthServiceApi};
use crate::common::responder::{ErrorResponse, SuccessResponse};

use abi::{LoginReq, LoginResponse, RefreshTokenResponse, RegisterReq, RegisterResponse, Token};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{post, routes, State};

#[post("/register", data = "<request>")]
async fn register_by_email(
    request: Json<RegisterReq>,
    auth_service: &State<AuthService>,
) -> Result<SuccessResponse<RegisterResponse>, ErrorResponse> {
    let response = auth_service.register_by_email(request.into_inner()).await?;
    Ok(SuccessResponse::Success(Json(response.into())))
}

#[post("/login", data = "<request>")]
async fn login_by_email(
    request: Json<LoginReq>,
    auth_service: &State<AuthService>,
) -> Result<SuccessResponse<LoginResponse>, ErrorResponse> {
    let response = auth_service.login_by_email(request.into_inner()).await?;
    Ok(SuccessResponse::Success(Json(response.into())))
}

#[post("/refresh_token", data = "<refresh_token>", format = "json")]
fn refresh_token(
    refresh_token: Token,
    auth_service: &State<AuthService>,
) -> Result<SuccessResponse<RefreshTokenResponse>, ErrorResponse> {
    let response = auth_service.refresh_token(refresh_token)?;
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
