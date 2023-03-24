use crate::auth_service::AuthorizedUser;
use crate::common::responder::{ErrorResponse, SuccessResponse};
use abi::{CreateFeedRequest, CreateFeedResponse, FecthFeedResponse, UserProfile};
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
    let feed_info = request.into_inner().feed_info;

    let feed_content = match feed_info {
        Some(info) => content_service.create_feed(info).await?,
        None => return Err(ErrorResponse::default()),
    };

    let user_feed = match feed_content.feed_profile.clone() {
        Some(profile) => user_service.create_user_feed(user_profile.clone(), profile),
        None => return Err(ErrorResponse::default()),
    };

    let user_content =
        user_service.create_user_content_multiple(user_profile, feed_content.contents.clone());

    Ok(SuccessResponse::Created(Json(CreateFeedResponse {
        feed_profile: feed_content.feed_profile,
        content: feed_content.contents,
        user_content: user_content.await?,
        user_feed: Some(user_feed.await?),
    })))
}

#[get("/")]
async fn get_feed_list(
    user: AuthorizedUser,
    user_service: &State<UserService>,
) -> Result<SuccessResponse<FecthFeedResponse>, ErrorResponse> {
    let user_profile: UserProfile = user.into();
    let user_feeds = user_service.query_user_feed(user_profile.id).await?;
    Ok(SuccessResponse::Success(Json(FecthFeedResponse {
        user_feeds,
    })))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Feed Source", |rocket| async {
        rocket.mount("/feed", routes![create_exist_feed, get_feed_list])
    })
}
