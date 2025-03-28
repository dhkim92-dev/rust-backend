use sea_orm::DatabaseConnection;
use crate::config;

struct AppState  {
    config: config::AppConfig,
    db:  DatabaseConnection
}
