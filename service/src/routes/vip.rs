use abi::{InternalError, PurchaseVerifyRequest, PurchaseVerifyResponse};
use axum::{debug_handler, extract::State, Json};
use user_service::UserServiceApi;

use crate::common::AppState;

#[debug_handler]
pub async fn purchase_verify(
    State(service): State<AppState>,
    Json(request): Json<PurchaseVerifyRequest>,
) -> Result<Json<PurchaseVerifyResponse>, InternalError> {
    let resp = service.user_service.purchase_verify(request).await?;
    Ok(Json(resp))
}
