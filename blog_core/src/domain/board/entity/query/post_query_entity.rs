use chrono::NaiveDateTime;

use crate::application::board::usecases::{WriterVo, CategoryVo};

pub struct QPostEntity {
    pub id: i32,
    pub writer: WriterVo,
    pub category: CategoryVo,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
