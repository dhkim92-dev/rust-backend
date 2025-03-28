use dotenvy::dotenv;
use std::env;

#[derive(clap::Parser, Debug)]
pub struct AppConfig {
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
}
