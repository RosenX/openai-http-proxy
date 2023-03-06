use futures::TryFutureExt;
use rocket::State;
use rocket::{fairing::AdHoc, routes, post};
use rocket::serde::json::{Json};
use sea_orm::{DatabaseConnection, ActiveValue, ActiveModelTrait};

use crate::common::responder::{SuccessResponse, ErrorResponse};
use crate::models::request::feed_info::ExistSourceInfo;
use crate::entities::{user_subscribe_source};
use crate::common::errors::InternalError;

use super::authorization::AuthorizedUser;

#[post("/add/exist", data = "<info>")]
async fn create_exist_source(
    user: AuthorizedUser,
    info: Json<ExistSourceInfo>,
    db: &State<DatabaseConnection>,
) ->  Result<SuccessResponse<String>, ErrorResponse>
{
    let info = info.into_inner();

    let source = info.create_exist_source(db).await?;
    user_subscribe_source::ActiveModel {
        user_id: ActiveValue::Set(user.user_id),
        subscribe_source_id: ActiveValue::Set(source.id),
        subscribe_source_name: ActiveValue::Set(info.name),
        subscribe_source_icon: ActiveValue::Set(info.icon),
        ..Default::default()
    }
    .insert(db.inner())
    .map_err(|err| InternalError::DatabaseError(err.to_string()))
    .await?;

    Ok(SuccessResponse::default_success_response())

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Describe Source Stage", |rocket| async {
        rocket.mount("/feed", routes![
            create_exist_source,
        ])
    })
}