use axum::Router;
use clap::Parser;
use common::database;
use tracing::info;
use std::env;
use  dotenvy::dotenv;

mod interfaces;
mod config;
mod common;

#[tokio::main]
async fn main() {
    let ctx = config::create_context().await;
    let app = config::api_router()
        .layer(axum::Extension(ctx));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listener");
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

