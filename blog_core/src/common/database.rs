use crate::config::AppConfig;
use sea_orm::{ConnectOptions, Database, DatabaseTransaction, DbConn, TransactionTrait};
use shaku::{Component, Interface};
use std::sync::Arc;
use tracing::{error, info};

#[async_trait::async_trait]
pub trait DbConnProvider: Interface {
    async fn ro_txn(&self) -> Result<DatabaseTransaction, sea_orm::DbErr>;

    async fn rw_txn(&self) -> Result<DatabaseTransaction, sea_orm::DbErr>;

    #[allow(dead_code)]
    async fn txn_with_options(
        &self,
        isolation_level: Option<sea_orm::IsolationLevel>,
        access_mode: Option<sea_orm::AccessMode>,
    ) -> Result<DatabaseTransaction, sea_orm::DbErr>;
}

#[derive(Component)]
#[shaku(interface = DbConnProvider)]
pub struct DbConnProviderImpl {
    db: DbConn,
}

#[async_trait::async_trait]
impl DbConnProvider for DbConnProviderImpl {
    async fn ro_txn(&self) -> Result<DatabaseTransaction, sea_orm::DbErr> {
        self.db
            .begin_with_config(
                Some(sea_orm::IsolationLevel::ReadCommitted),
                Some(sea_orm::AccessMode::ReadOnly),
            )
            .await
    }

    async fn rw_txn(&self) -> Result<DatabaseTransaction, sea_orm::DbErr> {
        self.db
            .begin_with_config(
                Some(sea_orm::IsolationLevel::ReadCommitted),
                Some(sea_orm::AccessMode::ReadWrite),
            )
            .await
    }

    async fn txn_with_options(
        &self,
        isolation_level: Option<sea_orm::IsolationLevel>,
        access_mode: Option<sea_orm::AccessMode>,
    ) -> Result<DatabaseTransaction, sea_orm::DbErr> {
        self.db
            .begin_with_config(isolation_level, access_mode)
            .await
    }
}

pub async fn init_db(config: Arc<AppConfig>) -> DbConn {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
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
