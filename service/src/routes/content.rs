use abi::{
    ContentPullRequest, ContentPullResponse, ContentPushRequest, ContentPushResponse,
    InternalError, Response, SubscribeFeedRequest, SubscribeFeedResponse,
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

pub async fn subscribe_feed(
    State(service): State<AppState>,
    user: AuthorizedUser,
    Json(request): Json<SubscribeFeedRequest>,
) -> Result<Json<SubscribeFeedResponse>, InternalError> {
    let response = service
        .content_service
        .subscribe_feed(user.get_user_id(), request)
        .await?;
    Ok(Json(response))
}

pub async fn subscribe_feed_v1(
    State(service): State<AppState>,
    user: AuthorizedUser,
    Json(request): Json<SubscribeFeedRequest>,
) -> Json<Response<()>> {
    let response = service
        .content_service
        .subscribe_feed(user.get_user_id(), request)
        .await;
    match response {
        Ok(_) => Json(Response::default()),
        Err(err) => Json(Response::from(err)),
    }
}
