use clap::Parser;
use dotenvy::dotenv;
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

    let app = interfaces::http::create_routers(Arc::new(ctx));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listener");
    info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
