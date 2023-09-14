pub mod content;
pub mod user;

use crate::common::AppState;
use axum::routing::{delete, get, post};
use axum::Router;
use content::{content_delete, subscribe_feed, sync_pull, sync_push};

use self::content::subscribe_feed_v1;
use self::user::user_activity;

fn content_routes() -> Router<AppState> {
    Router::new()
        .route("/pull", get(sync_pull))
        .route("/push", post(sync_push))
        .route("/subscribe", post(subscribe_feed))
}

fn v1_routes() -> Router<AppState> {
    Router::new()
        .nest("/feed", feed_routes_v1())
        .nest("/content", content_routes_v1())
        .nest("/user", user_routes_v1())
}

fn feed_routes_v1() -> Router<AppState> {
    Router::new().route("/subscribe", post(subscribe_feed_v1))
}

fn user_routes_v1() -> Router<AppState> {
    Router::new().route("/activity", post(user_activity))
}

fn content_routes_v1() -> Router<AppState> {
    Router::new().route("/delete", delete(content_delete))
}

pub fn create_route() -> Router<AppState> {
    Router::new()
        .nest("/content", content_routes())
        .nest("/v1", v1_routes())
}
