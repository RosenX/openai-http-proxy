use rocket::State;
use rocket::{fairing::AdHoc, routes, post};
use rocket::serde::json::{Json};
use crate::common::responder::{SuccessResponse, ErrorResponse};
use crate::database::DatabasePool;
use crate::models::request::feed_info::ExistSourceInfo;

use super::authorization::AuthorizedUser;

#[post("/add/exist", data = "<info>")]
async fn create_exist_source(
    user: AuthorizedUser,
    info: Json<ExistSourceInfo>,
    pool: &State<DatabasePool>,
) ->  Result<SuccessResponse<String>, ErrorResponse>
{
    let info = info.into_inner();
    info.create_user_feed(pool, user).await?;
    Ok(SuccessResponse::default_success_response())

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Describe Source Stage", |rocket| async {
        rocket.mount("/feed", routes![
            create_exist_source,
        ])
    })
}