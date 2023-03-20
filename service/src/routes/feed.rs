use crate::common::responder::{ErrorResponse, SuccessResponse};
use crate::database::user_feed::UserFeed;
use abi::{CreateFeedRequest, CreateFeedResponse, DbPool};
use content_service::{ContentService, ContentServiceApi};
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, post, routes};
use rocket::{get, State};

use super::authorization::AuthorizedUser;

#[post("/add/exist", data = "<request>")]
async fn create_exist_feed(
    _user: AuthorizedUser,
    request: Json<CreateFeedRequest>,
    content_service: &State<ContentService>,
) -> Result<SuccessResponse<CreateFeedResponse>, ErrorResponse> {
    let response = content_service.create_feed(request.into_inner()).await?;
    Ok(SuccessResponse::Created(Json(response)))
}

#[get("/")]
async fn get_feed_list(
    user: AuthorizedUser,
    pool: &State<DbPool>,
) -> Result<SuccessResponse<Vec<UserFeed>>, ErrorResponse> {
    let user_feed_list = UserFeed::retrieve_feed_by_user(user.id, pool.inner()).await?;
    Ok(SuccessResponse::Success(Json(user_feed_list)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Feed Source", |rocket| async {
        rocket.mount("/feed", routes![create_exist_feed, get_feed_list])
    })
}
