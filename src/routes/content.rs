use rocket::{fairing::AdHoc, routes, get, serde::Deserialize};
use rocket::serde::json::{Json};

use crate::common::errors::InternalError;
use crate::common::responder::{SuccessResponse, ErrorResponse};

use super::fetcher;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Uri {
    uri: String
}

#[get("/", data = "<uri>")]
async fn get_lastest_post(uri: Json<Uri>) -> Result<SuccessResponse<String>, ErrorResponse> {
    let result = fetcher::fetch_uri(&uri.uri)
        .await
        .map_err(|err| InternalError::InvalidUrl(err.to_string()))?;
    Ok(SuccessResponse::Success(Json(result)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Describe Source Stage", |rocket| async {
        rocket.mount("/content", routes![
            get_lastest_post,
        ])
    })
}