use std::sync::Arc;

use clap::Parser;
use shaku::{Component, Interface};

pub trait ConfigProvider: Interface {
    fn get(&self) -> Arc<AppConfig>;

    fn get_origin(&self) -> String;
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

    fn get_origin(&self) -> String {

        let protocol = if self.config.protocol == "https" {
            "https://"
        } else {
            "http://"
        };

        format!("{}{}", protocol, self.config.server_host)
    }
}

#[derive(Parser, Debug, Clone)]
pub struct AppConfig {
    #[arg(long, default_value = "dev")]
    pub app_env: String,
    // Server
    #[arg(long, default_value = "localhost:8080")]
    pub server_host: String,
    #[arg(long, default_value = "http")]
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
    // 100년
    // #[arg(long, default_value_t = 3153600000000)]
    #[arg(long, default_value_t = 900000)]
    pub jwt_access_token_expire: u64,
    //#[arg(long, default_value_t = 3153600000000)]
    #[arg(long, default_value_t = 604800000)]
    pub jwt_refresh_token_expire: u64,
    #[arg(long, default_value = "https://identification.dohoon-kim.kr")]
    pub jwt_issuer: String,
    #[arg(long, default_value = "https://www.dohoon-kim.kr")]
    pub jwt_audience: String,
    // OAuth
}

impl AppConfig {
    pub fn is_production(&self) -> bool {
        self.app_env == "prod"
    }
}
