use futures::TryFutureExt;
use rocket::State;
use rocket::serde::Deserialize;
use rocket::{fairing::AdHoc, routes, post};
use rocket::serde::json::{Json};
use sea_orm::{DatabaseConnection, ActiveValue, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait};

use crate::entities::{subscribe_source, user_subscribe_source};
use crate::entities::prelude::*;
use crate::utils::prelude::{ErrorResponse, SuccessResponse};

use super::authorization::AuthorizedUser;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ExistSourceInfo {
    uri: String,
    name: String,
    icon: String,
}

#[post("/exist", data = "<info>")]
async fn create_exist_source(
    user: AuthorizedUser,
    info: Json<ExistSourceInfo>,
    db: &State<DatabaseConnection>,
) ->  Result<SuccessResponse<String>, ErrorResponse>
{
    let info = info.into_inner();

    let source = SubscribeSource::find()
        .filter(subscribe_source::Column::UriIdentity.eq(info.uri.clone()))
        .one(db.inner())
        .await
        .map_err(|_| ErrorResponse::DefaultErrorResponse())?;

    let source = match source {
        None => {
            let source = subscribe_source::ActiveModel {
                uri_identity: ActiveValue::Set(info.uri),
                ..Default::default()
            }
            .insert(db.inner())
            .await
            .map_err(|_| ErrorResponse::DefaultErrorResponse())?;
            source
        },
        Some(source) => source,
    };
    user_subscribe_source::ActiveModel {
        user_id: ActiveValue::Set(user.user_id),
        subscribe_source_id: ActiveValue::Set(source.id),
        subscribe_source_name: ActiveValue::Set(info.name),
        subscribe_source_icon: ActiveValue::Set(info.icon),
        ..Default::default()
    }
    .insert(db.inner())
    .map_err(|_| ErrorResponse::DefaultErrorResponse())
    .await?;

    Ok(SuccessResponse::DefaultSuccessResponse())

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Stage", |rocket| async {
        rocket.mount("/source", routes![
            create_exist_source,
        ])
    })
}