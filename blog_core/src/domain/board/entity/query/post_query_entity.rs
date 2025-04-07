use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use uuid::Uuid;

use crate::application::board::usecases::{WriterVo, CategoryVo};

#[derive(Debug, Clone, FromQueryResult)]
pub struct QPostEntity {
    pub id: Uuid,
    #[sea_orm(nested)]
    pub writer: WriterVo,
    #[sea_orm(nested)]
    pub category: CategoryVo,
    pub title: String,
    pub contents: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
