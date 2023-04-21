#![deny(unused_crate_dependencies)]
mod auth_service;
mod common;
mod routes;

use common::{AppConfig, AppState};
use config::{Config, FileFormat};
use env_logger::Env;
use routes::create_route;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

fn load_config() -> AppConfig {
    let config = Config::builder()
        .add_source(config::File::with_name("config.yaml").format(FileFormat::Yaml))
        .build()
        .expect("Failed to load config.yaml");

    config
        .try_deserialize()
        .expect("Failed to deserialize config.yaml")
}

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "Info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let app_config: AppConfig = load_config();

    let app_state = AppState::new(app_config.auth_service, app_config.database)
        .await
        .expect("Failed to create app state");

    let app = create_route()
        .with_state(app_state)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let host = format!("{}:{}", app_config.server.ip, app_config.server.port);
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
