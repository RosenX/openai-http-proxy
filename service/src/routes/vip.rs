use abi::{InternalError, PurchaseVerifyRequest, PurchaseVerifyResponse, VipStatus};
use axum::{debug_handler, extract::State, Json};
use user_service::UserServiceApi;

use crate::common::{AppState, AuthorizedUser};

#[debug_handler]
pub async fn purchase_verify(
    user: AuthorizedUser,
    State(service): State<AppState>,
    Json(request): Json<PurchaseVerifyRequest>,
) -> Result<Json<PurchaseVerifyResponse>, InternalError> {
    let resp = service
        .user_service
        .purchase_verify(&user.get_user_id(), request)
        .await?;
    Ok(Json(resp))
}

#[debug_handler]
pub async fn get_vip_status(
    user: AuthorizedUser,
    State(service): State<AppState>,
) -> Result<Json<Option<VipStatus>>, InternalError> {
    let resp = service.user_service.vip_status(&user.get_user_id()).await?;
    Ok(Json(resp))
}
