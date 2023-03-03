use futures::TryFutureExt;
use rocket::State;
use rocket::serde::Deserialize;
use rocket::{fairing::AdHoc, routes, post};
use rocket::serde::json::{Json};
use sea_orm::{DatabaseConnection, ActiveValue, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait};

use crate::common::responder::{SuccessResponse, ErrorResponse};
use crate::entities::{subscribe_source, user_subscribe_source};
use crate::entities::prelude::*;
use crate::common::errors::InternalError;

use super::authorization::AuthorizedUser;

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct ExistSourceInfo {
    uri: String,
    name: String,
    icon: String,
}

impl From<ExistSourceInfo> for subscribe_source::ActiveModel {
    fn from(info: ExistSourceInfo) -> Self {
        Self {
            uri_identity: ActiveValue::Set(info.uri),
            ..Default::default()
        }
    }
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
        .map_err(|err| InternalError::SourceNotExist(err.to_string()))?;

    let source = match source {
        None => {
            let source:subscribe_source::ActiveModel = info.clone().into();
            let source = source
                .insert(db.inner()).await
                .map_err(|err| InternalError::DatabaseError(err.to_string()))?;
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
    .map_err(|err| InternalError::DatabaseError(err.to_string()))
    .await?;

    Ok(SuccessResponse::default_success_response())

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Describe Source Stage", |rocket| async {
        rocket.mount("/source", routes![
            create_exist_source,
        ])
    })
}