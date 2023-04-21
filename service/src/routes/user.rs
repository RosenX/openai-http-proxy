use crate::auth_service::AuthServiceApi;
use abi::{AuthResponse, InternalError, LoginRequest, RefreshTokenRequest, RegisterRequest};
use axum::extract::{Json, State};

use super::AppState;

pub async fn register_by_email(
    State(service): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, InternalError> {
    let response = service.auth_service.register_by_email(request).await?;
    Ok(Json(response))
}

pub async fn login_by_email(
    State(service): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, InternalError> {
    let response = service.auth_service.login_by_email(request).await?;
    Ok(Json(response))
}

pub async fn refresh_token(
    State(service): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, InternalError> {
    let response: AuthResponse = service.auth_service.refresh_token(request)?;
    Ok(Json(response))
}
