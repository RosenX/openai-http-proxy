#![deny(unused_crate_dependencies)]
pub mod common;
pub mod routes;

use axum::extract::DefaultBodyLimit;
use common::{AppConfig, AppState};
use config::{Config, FileFormat};
use routes::create_route;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
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
    // start timing
    let start = std::time::Instant::now();
    // Setup tracing
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .init();
    let app_config: AppConfig = load_config();

    // stat load config time
    let end = std::time::Instant::now();
    let elapsed = end.duration_since(start);
    info!("Config load time: {:?}", elapsed);

    let app_state = AppState::new(app_config.authing, app_config.database)
        .await
        .expect("Failed to create app state");

    // stat create app state time
    let end = std::time::Instant::now();
    let elapsed = end.duration_since(start);
    info!("App state create time: {:?}", elapsed);

    let app = create_route()
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true)),
        )
        .layer(DefaultBodyLimit::max(5242880)); // 5MB

    let host = format!("{}:{}", app_config.server.ip, app_config.server.port);

    // end timing
    let end = std::time::Instant::now();
    let elapsed = end.duration_since(start);
    info!("Startup time: {:?}", elapsed);

    info!("Starting server at: {}", host);
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
