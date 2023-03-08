use rocket::{catch, fairing::AdHoc, catchers};

use super::responder::{ErrorInfo, ErrorResponse};

#[catch(401)]
fn general_unauthorized() -> ErrorResponse {
    ErrorResponse::Unauthorized(ErrorInfo::new(None, "认证失败".to_string()).into())
}

#[catch(404)]
fn general_unfound() -> ErrorResponse {
    ErrorResponse::Unauthorized(ErrorInfo::new(None, "NotFound".to_string()).into())
}


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Cachers", |rocket| async {
        rocket.register("/", catchers![
            general_unauthorized,
            general_unfound
        ])
    })
}