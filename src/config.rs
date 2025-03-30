use std::sync::Arc;

use axum::{Extension, Router};
use dotenvy::dotenv;
use clap::Parser;
use crate::interfaces;

#[derive(clap::Parser, Debug, Clone)]
pub struct AppConfig {

    // Datasource
    #[arg(long,  default_value = "localhost")]
    pub database_host: String,
    #[arg(long,  default_value_t = 5432)]
    pub database_port: u16,
    #[arg(long,  default_value = "blog_admin")]
    pub database_username: String,
    #[arg(long,  default_value = "test1234")]
    pub database_password: String,
    #[arg(long,  default_value = "blog_dev")]
    pub database_name: String,
    #[arg(long,  default_value = "localhost")]
    pub redis_host: String,
    #[arg(long,  default_value_t = 6379)]
    pub redis_port: u16,

    // JWT 
    #[arg(long,  default_value = "test-access-token-secret")]
    pub jwt_access_token_secret: String,
    #[arg(long,  default_value = "test-refresh-token-secret")]
    pub jwt_refresh_token_secret: String,
    #[arg(long,  default_value_t=900000)]
    pub jwt_access_token_expire: u64,
    #[arg(long,  default_value_t=604800000)]
    pub jwt_refresh_token_expire: u64,
    #[arg(long,  default_value = "https://identification.dohoon-kim.kr")]
    pub jwt_issuer: String,
    #[arg(long,  default_value = "https://www.dohoon-kim.kr")]
    pub jwt_audience: String,
    // OAuth
}

#[derive(Debug, Clone)]
pub struct AppContext {
    pub config: std::sync::Arc<AppConfig>,
    pub db: sea_orm::DatabaseConnection
}
/* 
pub async fn create_context() -> AppContext{
    let env = std::env::var("ENV").unwrap_or_else(|_| "env".to_string());
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    if env == "dev" {
        dotenv().ok();
    }

    let app_config = AppConfig::try_parse()
        .unwrap_or_else(|_| {
            tracing::error!("Failed to parse config");
            std::process::exit(1);
        });

    let db = database::init_db(app_config.clone()).await;

    Arc::new(AppContext {
        config: std::sync::Arc::new(app_config),
        db: db
    })
} */
