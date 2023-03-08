use rocket::{State, get};
use rocket::{fairing::AdHoc, routes, post};
use rocket::serde::json::{Json};
use crate::common::config::common::CommonConfig;
use crate::common::responder::{SuccessResponse, ErrorResponse};
use crate::database::DatabasePool;
use crate::database::user_feed::UserFeed;
use crate::models::request::feed_info::FeedInfo;

use super::authorization::AuthorizedUser;

#[post("/add/exist", data = "<info>")]
async fn create_exist_feed(
    user: AuthorizedUser,
    info: Json<FeedInfo>,
    pool: &State<DatabasePool>,
    common_config: &State<CommonConfig>,
) ->  Result<SuccessResponse<FeedInfo>, ErrorResponse>
{
    let mut info: FeedInfo = info.into_inner();
    info.complete_info(&common_config);
    info.create_user_feed(pool, user).await?;
    Ok(SuccessResponse::Created(Json(info)))
}


#[get("/")]
async fn get_feed_list(
    user: AuthorizedUser,
    pool: &State<DatabasePool>,
) ->  Result<SuccessResponse<Vec<FeedInfo>>, ErrorResponse>
{
    let user_feed_list = UserFeed::retrieve_feed_by_user(user.user_id, pool.inner()).await?;
    let feed_list: Vec<FeedInfo> = user_feed_list.iter().map(|feed
        | FeedInfo::from(feed.to_owned())).collect();
    Ok(SuccessResponse::Success(Json(feed_list)))
}


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Feed Source", |rocket| async {
        rocket.mount("/feed", routes![
            create_exist_feed,
            get_feed_list
        ])
    })
}