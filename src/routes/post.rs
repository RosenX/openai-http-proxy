use rocket::{fairing::AdHoc, get, routes, State};
use rocket::serde::json::{Json};
use crate::database::user_custom_post::UserCustomPost;
use crate::{
    common::responder::{ErrorResponse, SuccessResponse},
    database::{DatabasePool}, models::request::post_req::PostReq,
};

use super::authorization::AuthorizedUser;

#[get("/pull", data = "<req>")]
async fn get_lastest_post(
    user: AuthorizedUser,
    req: Json<PostReq>,
    pool: &State<DatabasePool>,
) -> Result<SuccessResponse<Vec<UserCustomPost>>, ErrorResponse> {
    let posts = UserCustomPost::retrieve_lastest_post(pool, user.id, req.latest_post_id).await?;
    Ok(SuccessResponse::Success(Json(posts)))
}

#[get("/pull/<feed_id>", data = "<req>")]
async fn get_lastest_post_by_id(
    user: AuthorizedUser,
    req: Json<PostReq>,
    pool: &State<DatabasePool>,
    feed_id: i32,
) -> Result<SuccessResponse<Vec<UserCustomPost>>, ErrorResponse> {
    let posts = UserCustomPost::retrieve_lastest_post_by_id(pool, user.id, req.latest_post_id, feed_id).await?;
    Ok(SuccessResponse::Success(Json(posts)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Content", |rocket| async {
        rocket
            .mount("/post", routes![get_lastest_post,])
            .mount("/post", routes![get_lastest_post_by_id,])
    })
}
