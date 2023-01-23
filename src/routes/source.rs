use rocket::State;
use rocket::serde::Deserialize;
use rocket::{fairing::AdHoc, routes, post};
use rocket::serde::json::{Json};
use sea_orm::{DatabaseConnection, ActiveValue, ActiveModelTrait};

use crate::entities::{subscribe_source, user_subscribe_source};
use crate::utils::responder::{SuccessJsonResponder, FailureJsonResponder, BodyData};

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
) ->  Result<SuccessJsonResponder<String>, FailureJsonResponder<String>>
{
    let info = info.into_inner();

    let source = subscribe_source::ActiveModel {
        uri_identity: ActiveValue::Set(info.uri),
        ..Default::default()
    };

    let source = source.insert(db.inner()).await?;

    let user_source = user_subscribe_source::ActiveModel {
        user_id: ActiveValue::Set(user.user_id),
        subscribe_source_id: ActiveValue::Set(source.id),
        subscribe_source_name: ActiveValue::Set(info.name),
        subscribe_source_icon: ActiveValue::Set(info.icon),
        ..Default::default()
    };

    let user_source = user_source.insert(db.inner()).await?;
    
    Ok(BodyData{data: "Success".to_string()}.into())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Stage", |rocket| async {
        rocket.mount("/source", routes![
            create_exist_source,
        ])
    })
}