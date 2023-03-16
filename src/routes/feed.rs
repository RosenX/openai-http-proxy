use crate::common::responder::{ErrorResponse, SuccessResponse};
use crate::common::service::feed_service::FeedService;
use crate::common::service::http_service::HttpService;
use crate::common::service::mysql_service::MySqlService;
use crate::database::feed_post::{FeedPost};
use crate::database::feed_profile::{FeedProfile};
use crate::database::user_feed::UserFeed;
use crate::models::request::feed_req::FeedReq;
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, post, routes};
use rocket::{get, State};

use super::authorization::AuthorizedUser;

#[post("/add/exist", data = "<info>")]
async fn create_exist_feed(
    user: AuthorizedUser,
    info: Json<FeedReq>,
    pool: &State<MySqlService>,
    feed_service: &State<FeedService>,
    http: &State<HttpService>,
) -> Result<SuccessResponse<UserFeed>, ErrorResponse> {
    let info: FeedReq = info.into_inner();

    let feed = FeedService::fetch_from_url(http, &info.url).await?;
    let mut feed_profile = FeedProfile::new(&feed, info, feed_service);
    let feed_profile = feed_profile.insert(pool.inner()).await?;
    let mut feed_post_list: Vec<FeedPost> = feed
        .entries
        .iter()
        .map(|entry| FeedPost::new(entry, &feed_profile, feed_service))
        .collect();
    for feed_post in &mut feed_post_list {
        feed_post.insert(pool).await?;
    }
    let user_feed = UserFeed::new(feed_profile, user);
    user_feed.insert(pool).await?;
    Ok(SuccessResponse::Created(Json(user_feed)))
}

#[get("/")]
async fn get_feed_list(
    user: AuthorizedUser,
    pool: &State<MySqlService>,
) -> Result<SuccessResponse<Vec<UserFeed>>, ErrorResponse> {
    let user_feed_list = UserFeed::retrieve_feed_by_user(user.id, pool.inner()).await?;
    Ok(SuccessResponse::Success(Json(user_feed_list)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Feed Source", |rocket| async {
        rocket.mount("/feed", routes![create_exist_feed, get_feed_list])
    })
}
