use rocket::{State, get};
use rocket::{fairing::AdHoc, routes, post};
use rocket::serde::json::{Json};
use crate::common::config::common::CommonConfig;
use crate::common::responder::{SuccessResponse, ErrorResponse};
use crate::database::DatabasePool;
use crate::database::feed_profile::FeedProfile;
use crate::database::user_feed::UserFeed;
use crate::models::request::feed_req::FeedReq;

use super::authorization::AuthorizedUser;

#[post("/add/exist", data = "<info>")]
async fn create_exist_feed(
    user: AuthorizedUser,
    info: Json<FeedReq>,
    pool: &State<DatabasePool>,
    common_config: &State<CommonConfig>,
) ->  Result<SuccessResponse<UserFeed>, ErrorResponse>
{
    let info: FeedReq = info.into_inner();
    let mut feed = FeedProfile::new(info, common_config).await?;
    let feed = feed.create_feed(pool.inner()).await?;
    let user_feed = UserFeed::new(feed, user);
    user_feed.insert(pool).await?;
    Ok(SuccessResponse::Created(Json(user_feed)))
}


#[get("/")]
async fn get_feed_list(
    user: AuthorizedUser,
    pool: &State<DatabasePool>,
) ->  Result<SuccessResponse<Vec<UserFeed>>, ErrorResponse>
{
    let user_feed_list = UserFeed::retrieve_feed_by_user(user.id, pool.inner()).await?;
    Ok(SuccessResponse::Success(Json(user_feed_list)))
}


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Feed Source", |rocket| async {
        rocket.mount("/feed", routes![
            create_exist_feed,
            get_feed_list
        ])
    })
}