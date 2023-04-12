use abi::{ContentPullRequest, ContentPullResponse, ContentPushRequest, ContentPushResponse};
use content_sync::ContentSyncService;
use rocket::{fairing::AdHoc, get, post, routes, serde::json::Json, State};

use crate::{
    auth_service::AuthorizedUser,
    common::responder::{ErrorResponse, SuccessResponse},
};

// #[get("/pull/<feed_id>", data = "<req>")]
// async fn get_latest_post_by_id(
//     user: AuthorizedUser,
//     req: Json<PostReq>,
//     pool: &State<MySqlService>,
//     feed_id: i32,
// ) -> Result<SuccessResponse<Vec<UserPost>>, ErrorResponse> {
//     let posts =
//         UserPost::retrieve_latest_post_by_id(pool, user.id, req.latest_post_id, feed_id)
//             .await?;
//     Ok(SuccessResponse::Success(Json(posts)))
// }

#[get("/pull", data = "<request>")]
async fn sync_pull(
    user: AuthorizedUser,
    service: &State<ContentSyncService>,
    request: Json<ContentPullRequest>,
) -> Result<SuccessResponse<ContentPullResponse>, ErrorResponse> {
    todo!();
}

#[post("/push", data = "<request>")]
async fn sync_push(
    user: AuthorizedUser,
    service: &State<ContentSyncService>,
    request: Json<ContentPushRequest>,
) -> Result<SuccessResponse<ContentPushResponse>, ErrorResponse> {
    todo!();
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite(
        "Loading Routes About Content Sync Service",
        |rocket| async { rocket.mount("/Content", routes![sync_pull]) },
    )
}
