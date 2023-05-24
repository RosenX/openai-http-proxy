pub mod content;
pub mod user;

use crate::common::AppState;
use axum::routing::{delete, get, post};
use axum::Router;
use content::{subscribe_feed, sync_pull, sync_push};
use user::{destroy_account, login_by_email, refresh_token, register_by_email};

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_by_email))
        .route("/register", post(register_by_email))
        .route("/refresh_token", post(refresh_token))
        .route("/destroy", delete(destroy_account))
}

fn content_routes() -> Router<AppState> {
    Router::new()
        .route("/pull", get(sync_pull))
        .route("/push", post(sync_push))
        .route("/subscribe", post(subscribe_feed))
}

pub fn create_route() -> Router<AppState> {
    Router::new()
        .nest("/user", user_routes())
        .nest("/content", content_routes())
}
