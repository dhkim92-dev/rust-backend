use sea_orm::{ConnectOptions, Database, DbConn};
use shaku::{Component, Interface};
use tracing::{info, error};
use std::sync::Arc;
use crate::config::AppConfig;

pub trait DbConnProvider: Interface {
    pub fn get() -> DbConn;
}

#[derive(Component)]
#[shaku(interface = DbConnProvider)]
pub struct DbConnProviderImpl {
    db: DbConn,
}


pub async fn init_db(config: Arc<AppConfig>) -> DbConn {
    let database_url = format!("postgres://{}:{}@{}:{}/{}", 
        config.database_username, 
        config.database_password, 
        config.database_host, 
        config.database_port, 
        config.database_name
    );

    info!("Connecting to database at {}", database_url);

    let mut opt = ConnectOptions::new(database_url.clone());
    opt.max_connections(64);
    opt.min_connections(4);
    opt.sqlx_logging(true);
    opt.sqlx_logging_level(tracing::log::LevelFilter::Info);
    opt.connect_timeout(std::time::Duration::from_secs(10));
    opt.max_lifetime(std::time::Duration::from_secs(10));
    opt.idle_timeout(std::time::Duration::from_secs(10));

    match Database::connect(opt).await {
        Ok(conn) => {
            info!("Connected to database");
            conn
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);    
            panic!("Failed to connect to database: {}", e);
        }
    }
}

