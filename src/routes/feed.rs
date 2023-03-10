use crate::common::config::common::CommonConfig;
use crate::common::errors::InternalError;
use crate::common::responder::{ErrorResponse, SuccessResponse};
use crate::common::service::feed_parser::FeedParser;
use crate::common::service::http_service::HttpService;
use crate::database::feed_profile::FeedProfile;
use crate::database::user_custom_feed::UserCustomFeed;
use crate::database::DatabasePool;
use crate::models::request::feed_req::FeedReq;
use feed_rs::parser;
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, post, routes};
use rocket::{get, State};

use super::authorization::AuthorizedUser;

#[post("/add/exist", data = "<info>")]
async fn create_exist_feed(
    user: AuthorizedUser,
    info: Json<FeedReq>,
    pool: &State<DatabasePool>,
    common_config: &State<CommonConfig>,
    http: &State<HttpService>
) -> Result<SuccessResponse<UserCustomFeed>, ErrorResponse> {
    let info: FeedReq = info.into_inner();
    
    let data = FeedParser::fetch_from_url(http, &info.url).await?;
    let feed = parser::parse(data.as_bytes()).map_err(|e| InternalError::FeedParseError(e))?;

    let mut feed = FeedProfile::new(feed, info, common_config).await?;
    let feed = feed.create_feed(pool.inner()).await?;
    let user_feed = UserCustomFeed::new(feed, user);
    user_feed.insert(pool).await?;
    Ok(SuccessResponse::Created(Json(user_feed)))
}

#[get("/")]
async fn get_feed_list(
    user: AuthorizedUser,
    pool: &State<DatabasePool>,
) -> Result<SuccessResponse<Vec<UserCustomFeed>>, ErrorResponse> {
    let user_feed_list = UserCustomFeed::retrieve_feed_by_user(user.id, pool.inner()).await?;
    Ok(SuccessResponse::Success(Json(user_feed_list)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Feed Source", |rocket| async {
        rocket.mount("/feed", routes![create_exist_feed, get_feed_list])
    })
}
