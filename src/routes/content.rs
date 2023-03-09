use rocket::{fairing::AdHoc, routes, get, serde::Deserialize};
use rocket::serde::json::{Json};

use crate::common::responder::{SuccessResponse, ErrorResponse};
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Uri {
    uri: String
}

#[get("/", data = "<uri>")]
async fn get_lastest_post(uri: Json<Uri>) -> Result<SuccessResponse<String>, ErrorResponse> {
    todo!();
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Content", |rocket| async {
        rocket.mount("/content", routes![
            get_lastest_post,
        ])
    })
}