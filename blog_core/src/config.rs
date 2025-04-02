use std::sync::Arc;

// use crate::interfaces;
// use axum::{Extension, Router};
use clap::Parser;
// use dotenvy::dotenv;
use shaku::{Component, Interface};

pub trait ConfigProvider: Interface {
    fn get(&self) -> Arc<AppConfig>;
}

#[derive(Component)]
#[shaku(interface = ConfigProvider)]
pub struct ConfigProviderImpl {
    pub config: Arc<AppConfig>,
}

impl ConfigProvider for ConfigProviderImpl {
    fn get(&self) -> Arc<AppConfig> {
        self.config.clone()
    }
}

#[derive(Parser, Debug, Clone)]
pub struct AppConfig {
    // Server
    #[arg(long, default_value = "localhost:8080")]
    pub server_host: String,
    #[arg(long, default_value = "HTTP")]
    protocol: String,
    // Datasource
    #[arg(long, default_value = "localhost")]
    pub database_host: String,
    #[arg(long, default_value_t = 5432)]
    pub database_port: u16,
    #[arg(long, default_value = "blog_admin")]
    pub database_username: String,
    #[arg(long, default_value = "test1234")]
    pub database_password: String,
    #[arg(long, default_value = "blog_dev")]
    pub database_name: String,
    #[arg(long, default_value = "localhost")]
    pub redis_host: String,
    #[arg(long, default_value_t = 6379)]
    pub redis_port: u16,

    // JWT
    #[arg(long, default_value = "test-access-token-secret")]
    pub jwt_access_token_secret: String,
    #[arg(long, default_value = "test-refresh-token-secret")]
    pub jwt_refresh_token_secret: String,
    #[arg(long, default_value_t = 900000)]
    pub jwt_access_token_expire: u64,
    #[arg(long, default_value_t = 604800000)]
    pub jwt_refresh_token_expire: u64,
    #[arg(long, default_value = "https://identification.dohoon-kim.kr")]
    pub jwt_issuer: String,
    #[arg(long, default_value = "https://www.dohoon-kim.kr")]
    pub jwt_audience: String,
    // OAuth
}
