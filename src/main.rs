use axum::{middleware::AddExtension, Extension, Router};
use clap::Parser;
use tracing::info;
use std::{env, sync::Arc};
use  dotenvy::dotenv;

mod interfaces;
mod common;
mod config;

use common::database;
use config::{AppConfig, AppContext};


#[tokio::main]
async fn main() {
    dotenv().ok();
    let cfg = AppConfig::try_parse()
        .unwrap_or_else(|_| {
            AppConfig::parse_from(env::args())
        });
    
    let db = database::init_db(&cfg).await;
    let ctx = AppContext {
        config: Arc::new(cfg.clone()),
        db
    };

    let app = interfaces::http::create_routers(ctx.clone());

    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listener");
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

