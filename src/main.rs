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

 //   if env::var("APP_ENV").unwrap() == "dev" {
 //       dotenv().ok();
  //  }

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = config::AppConfig::try_parse()
        .unwrap_or_else(|_| {
            tracing::error!("Failed to parse config");
            std::process::exit(1);
        });

    info!("Config: {:?}", config);

    let app = Router::new()
        .nest("/api/v1/auth", interfaces::auth::controller::create_router());

    let conn = database::init_db(config).await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listener");
    println!("Listening on {}", listener.local_addr().unwrap());


    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

