use crate::common::service::mysql_service::MySqlService;
use crate::database::user_post::UserPost;
use crate::{
    common::responder::{ErrorResponse, SuccessResponse},
    models::request::post_req::PostReq,
};
use log::info;
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, get, routes, State};

use super::authorization::AuthorizedUser;

#[get("/pull?<req..>")]
async fn get_lastest_post(
    user: AuthorizedUser,
    req: PostReq,
    pool: &State<MySqlService>,
) -> Result<SuccessResponse<Vec<UserPost>>, ErrorResponse> {
    info!("{}, {}", user.id, req.latest_post_id);
    let posts = UserPost::retrieve_lastest_post(pool, user.id, req.latest_post_id).await?;
    Ok(SuccessResponse::Success(Json(posts)))
}

#[get("/pull/<feed_id>", data = "<req>")]
async fn get_lastest_post_by_id(
    user: AuthorizedUser,
    req: Json<PostReq>,
    pool: &State<MySqlService>,
    feed_id: i32,
) -> Result<SuccessResponse<Vec<UserPost>>, ErrorResponse> {
    let posts =
        UserPost::retrieve_lastest_post_by_id(pool, user.id, req.latest_post_id, feed_id)
            .await?;
    Ok(SuccessResponse::Success(Json(posts)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Post", |rocket| async {
        rocket.mount("/post", routes![get_lastest_post, get_lastest_post_by_id])
    })
}
