use axum::http::HeaderValue;
use clap::Parser;
use dotenvy::dotenv;
use reqwest::{header, Method};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use std::{env, sync::Arc};
use tracing::info;
mod application;
mod common;
mod config;
mod di;
mod domain;
mod interfaces;

use common::database;
use config::AppConfig;
use config::{ConfigProviderImpl, ConfigProviderImplParameters};
use di::AppContext;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let cfg = AppConfig::try_parse().unwrap_or_else(|_| AppConfig::parse_from(env::args()));
    let cfg: Arc<AppConfig> = Arc::new(cfg);
    let db = database::init_db(cfg.clone()).await;

    let ctx = AppContext::builder()
        .with_component_parameters::<database::DbConnProviderImpl>(
            database::DbConnProviderImplParameters { db: db },
        )
        .with_component_parameters::<ConfigProviderImpl>(ConfigProviderImplParameters {
            config: cfg.clone(),
        })
        .build();

    let cors_layers = get_cors_layers(cfg);
    let app = interfaces::http::create_routers(Arc::new(ctx))
        .layer(cors_layers);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listener");
    info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

fn get_cors_layers(cfg: Arc<AppConfig>) -> CorsLayer {

    let mut origin: Vec<&str> = Vec::new();
    origin.push(cfg.server_host.as_str());
    if cfg.app_env == "dev" {
        origin.push("http://localhost:3000");
    }

    let origin = origin.iter().map(|&s| {
        HeaderValue::from_str(s).unwrap_or_else(|_| {
            panic!("Invalid header value: {}", s)
        })
    }).collect::<Vec<_>>();

    let layer = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::ORIGIN,
            header::ACCEPT,
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static("x-requested-with"),
        ])
        .allow_credentials(true);

    layer
}
