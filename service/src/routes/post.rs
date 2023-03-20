use crate::database::user_post::UserPost;
use crate::{
    common::responder::{ErrorResponse, SuccessResponse},
    models::request::post_req::PostReq,
};
use abi::DbPool;
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, get, routes, State};

use super::auth_service::AuthorizedUser;

// #[get("/pull?<req..>")]
// async fn get_latest_post(
//     user: AuthorizedUser,
//     req: PostReq,
//     pool: &State<DbPool>,
// ) -> Result<SuccessResponse<Vec<UserPost>>, ErrorResponse> {
//     let posts = UserPost::retrieve_latest_post(pool, user.id, req.latest_post_id).await?;
//     Ok(SuccessResponse::Success(Json(posts)))
// }

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

// pub fn stage() -> AdHoc {
//     AdHoc::on_ignite("Loading Routes About Post", |rocket| async {
//         rocket.mount("/post", routes![get_latest_post])
//     })
// }
