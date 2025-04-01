use clap::Parser;
use tracing::info;
use std::{env, sync::Arc};
use dotenvy::dotenv;
mod interfaces;
mod common;
mod config;
mod domain;
mod application;
mod di;

use common::database;
use config::AppConfig;
use di::AppContext;
use domain::member::repository::{MemberQueryRepository, MemberQueryRepositoryParameters};
use config::{ConfigProviderImpl, ConfigProviderImplParameters};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cfg = AppConfig::try_parse()
        .unwrap_or_else(|_| {
            AppConfig::parse_from(env::args())
        });
    let cfg: Arc<AppConfig> = Arc::new(cfg);
    let db = Arc::new(database::init_db(cfg.clone()).await);
    let ctx = AppContext::builder()
        .with_component_parameters::<ConfigProviderImpl>(ConfigProviderImplParameters {
            config: cfg.clone()
        })
    .build();
    
    let app = interfaces::http::create_routers(Arc::new(ctx), db);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listener");
    info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

