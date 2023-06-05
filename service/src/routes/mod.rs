pub mod content;
pub mod user;

use crate::common::AppState;
use axum::routing::{delete, get, post};
use axum::Router;
use content::{subscribe_feed, sync_pull, sync_push};
use user::{destroy_account, login_by_email, modify_password, refresh_token, register_by_email};

use self::content::subscribe_feed_v1;
use self::user::login_by_email_v1;

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_by_email))
        .route("/register", post(register_by_email))
        .route("/refresh_token", post(refresh_token))
        .route("/destroy", delete(destroy_account))
        .route("/modify_password", post(modify_password))
}

fn content_routes() -> Router<AppState> {
    Router::new()
        .route("/pull", get(sync_pull))
        .route("/push", post(sync_push))
        .route("/subscribe", post(subscribe_feed))
}

fn v1_routes() -> Router<AppState> {
    Router::new()
        .nest("/user", user_routes_v1())
        .nest("/feed", feed_routes_v1())
}

fn user_routes_v1() -> Router<AppState> {
    Router::new().route("/login", post(login_by_email_v1))
}

fn feed_routes_v1() -> Router<AppState> {
    Router::new().route("/subscribe", post(subscribe_feed_v1))
}

pub fn create_route() -> Router<AppState> {
    Router::new()
        .nest("/user", user_routes())
        .nest("/content", content_routes())
        .nest("/v1", v1_routes())
}
