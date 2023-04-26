#![deny(unused_crate_dependencies)]
pub mod auth_service;
pub mod common;
pub mod routes;

use common::{AppConfig, AppState};
use config::{Config, FileFormat};
use routes::create_route;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

fn load_config() -> AppConfig {
    // Read 'FEEDBOX_ENV' from environment variable
    let env =
        std::env::var("FEEDBOX_ENV").expect("Please set FEEDBOX_ENV to 'dev' or 'prod' or 'test'");

    // set config file name according to environment
    let config_file = match env.as_str() {
        "dev" => "config.dev.yaml",
        "prod" => "config.prod.yaml",
        "test" => "config.test.yaml",
        _ => panic!("Please set FEEDBOX_ENV to 'dev' or 'prod'"),
    };

    info!("Loading config file: {}", config_file);

    let config = Config::builder()
        .add_source(config::File::with_name(config_file).format(FileFormat::Yaml))
        .build()
        .expect("Failed to load config.yaml");

    config
        .try_deserialize()
        .expect("Failed to deserialize config.yaml")
}

#[tokio::main]
async fn main() {
    // Setup tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let app_config: AppConfig = load_config();

    let app_state = AppState::new(app_config.auth_service, app_config.database)
        .await
        .expect("Failed to create app state");

    let layer = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

    let app = create_route().with_state(app_state).layer(layer);

    let host = format!("{}:{}", app_config.server.ip, app_config.server.port);
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
