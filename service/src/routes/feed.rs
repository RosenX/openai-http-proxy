use crate::auth_service::AuthorizedUser;
use crate::common::responder::{ErrorResponse, SuccessResponse};
use abi::{CreateFeedRequest, CreateFeedResponse, UserFeedResponse, UserProfile};
use content_service::{ContentService, ContentServiceApi};
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, post, routes};
use rocket::{get, State};
use user_service::{UserService, UserServiceApi};

#[post("/add/exist", data = "<request>")]
async fn create_exist_feed(
    user: AuthorizedUser,
    request: Json<CreateFeedRequest>,
    content_service: &State<ContentService>,
    user_service: &State<UserService>,
) -> Result<SuccessResponse<CreateFeedResponse>, ErrorResponse> {
    let user_profile: UserProfile = user.into();
    let feed_response = content_service.create_feed(request.into_inner()).await?;
    let user_feed =
        user_service.create_user_feed(user_profile.clone(), feed_response.feed_profile.clone());
    let user_content =
        user_service.create_user_content_multiple(user_profile, feed_response.contents.clone());

    Ok(SuccessResponse::Created(Json(CreateFeedResponse {
        feed_profile: feed_response.feed_profile,
        content: feed_response.contents,
        user_content: user_content.await?,
        user_feed: user_feed.await?,
    })))
}

#[get("/")]
async fn get_feed_list(
    user: AuthorizedUser,
    user_service: &State<UserService>,
) -> Result<SuccessResponse<UserFeedResponse>, ErrorResponse> {
    let user_profile: UserProfile = user.into();
    let feed_list = user_service.query_user_feed(user_profile.id).await?;
    Ok(SuccessResponse::Success(Json(UserFeedResponse {
        feed_list,
    })))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Feed Source", |rocket| async {
        rocket.mount("/feed", routes![create_exist_feed, get_feed_list])
    })
}
