use crate::{
    auth_service::AuthorizedUser,
    common::responder::{ErrorResponse, SuccessResponse},
};

use abi::{ContentId, UserContentResponse, UserProfile};
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, get, routes, State};
use user_service::{UserService, UserServiceApi};

#[get("/pull?<last_content_id>")]
async fn get_latest_post(
    user: AuthorizedUser,
    last_content_id: ContentId,
    user_service: &State<UserService>,
) -> Result<SuccessResponse<UserContentResponse>, ErrorResponse> {
    let user_profile: UserProfile = user.into();
    let content_list = user_service
        .query_latest_content(user_profile.id, last_content_id)
        .await?;
    Ok(SuccessResponse::Success(Json(UserContentResponse {
        content_list,
    })))
}

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

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Post", |rocket| async {
        rocket.mount("/post", routes![get_latest_post])
    })
}
