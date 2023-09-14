use abi::{InternalError, UserActivityRequest};
use axum::{extract::State, Json};
use user_service::UserServiceApi;

use crate::common::AppState;

pub async fn user_activity(
    State(service): State<AppState>,
    Json(request): Json<UserActivityRequest>,
) -> Result<(), InternalError> {
    service.user_service.user_activity(request).await
}
