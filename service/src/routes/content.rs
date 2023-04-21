use abi::{
    ContentPullRequest, ContentPullResponse, ContentPushRequest, ContentPushResponse, InternalError,
};
use axum::extract::{Json, State};
use content_sync::ContentSyncServiceApi;

use crate::auth_service::AuthorizedUser;

use super::AppState;

pub async fn sync_pull(
    user: AuthorizedUser,
    State(service): State<AppState>,
    Json(request): Json<ContentPullRequest>,
) -> Result<Json<ContentPullResponse>, InternalError> {
    let response = service
        .content_service
        .pull(user.get_user_id(), request)
        .await?;
    Ok(Json(response))
}

pub async fn sync_push(
    State(service): State<AppState>,
    user: AuthorizedUser,
    Json(request): Json<ContentPushRequest>,
) -> Result<Json<ContentPushResponse>, InternalError> {
    let response = service
        .content_service
        .push(user.get_user_id(), request)
        .await?;
    Ok(Json(response))
}
