use crate::auth_service::{AuthServiceApi, AuthorizedUser};
use abi::{
    AuthResponse, InternalError, LoginRequest, ModifyPasswordRequest, RefreshTokenRequest,
    RegisterRequest,
};
use axum::extract::{Json, State};
use content_sync::ContentSyncServiceApi;

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

pub async fn destroy_account(
    State(service): State<AppState>,
    user: AuthorizedUser,
) -> Result<(), InternalError> {
    let user_delete_future = service.auth_service.delete_user_account(user.get_user_id());
    let content_delete_future = service.content_service.delete(user.get_user_id());
    tokio::try_join!(user_delete_future, content_delete_future)?;
    Ok(())
}

pub async fn modify_password(
    State(service): State<AppState>,
    Json(request): Json<ModifyPasswordRequest>,
) -> Result<(), InternalError> {
    service.auth_service.modify_password(request).await?;
    Ok(())
}
