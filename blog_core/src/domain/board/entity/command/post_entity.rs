use chrono::{NaiveDateTime};
use crate::common::error_code::ErrorCode;
use uuid::Uuid;

use crate::common::AppError;

pub struct PostEntity {
    id: Option<Uuid>,
    category_id: i64,
    member_id: uuid::Uuid,
    title: String,
    content: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl PostEntity {
    fn new(
        id: Option<uuid::Uuid>,
        category_id: i64,
        member_id: Uuid,
        title: String,
        contents: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        PostEntity {
            id: id,
            category_id: category_id,
            member_id: member_id,
            title: title.to_owned(),
            content: contents.to_owned(),
            created_at: created_at.unwrap_or(chrono::Utc::now().naive_utc()),
            updated_at: updated_at,
        }
    }

    pub fn validate(&self) -> Result<bool, AppError> {
        self.validate_title(&self.title)?;

        return Ok(true)
    }

    fn validate_title(&self, value: &String) -> Result<bool, AppError> {
        if value.is_empty() {
            return Err(AppError::with_message(ErrorCode::ValidationError, "Title cannot be empty"))
        }

        if value.chars().count() > 255 {
            return Err(AppError::with_message(ErrorCode::ValidationError, "Title cannot be more than 255 characters"))
        }
        Ok(true)
    }
}
