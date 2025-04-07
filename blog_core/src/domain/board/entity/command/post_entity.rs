use chrono::NaiveDateTime;
use derive_builder::Builder;
use crate::common::error_code::ErrorCode;
use uuid::Uuid;

use crate::common::AppError;

#[derive(Debug, Clone, Builder)]
pub struct PostEntity {
    id: Option<Uuid>,
    category_id: i64,
    member_id: uuid::Uuid,
    title: String,
    contents: String,
    view_count: i64,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl PostEntity {
    pub fn new(
        id: Option<Uuid>,
        category_id: i64,
        member_id: Uuid,
        title: String,
        contents: String,
        view_count: i64,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        PostEntity {
            id,
            category_id,
            member_id,
            title: title.to_owned(),
            contents: contents.to_owned(),
            view_count,
            created_at: created_at.unwrap_or(chrono::Utc::now().naive_utc()),
            updated_at,
        }
    }

    pub fn check_ownership(&self, member_id: Uuid) -> Result<(), AppError> {
        if self.member_id != member_id {
            return Err(AppError::with_message(ErrorCode::Forbidden, "게시글 소유자가 아닙니다."));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<bool, AppError> {
        if !Self::validate_title(&self.title) {
            return Err(AppError::with_message(ErrorCode::BadRequest, "게시글 제목은 5자 이상 255자 이하로 작성해야 합니다."));
        }

        if !Self::validate_contents(&self.contents) {
            return Err(AppError::with_message(ErrorCode::BadRequest, "게시글 본문은 5자 이상 65535자 이하로 작성해야 합니다."));
        }

        return Ok(true)
    }

    pub fn validate_title(value: &String) -> bool {
        let sz_chrs = value.chars().count();
         !(value.is_empty () || sz_chrs <= 4 || sz_chrs > 255)
    }

    pub fn validate_contents(value: &String) -> bool {
        let sz_chrs = value.chars().count();

        !(value.is_empty () || sz_chrs <= 4 || sz_chrs > 65535)
    }

    pub fn change_contents(&mut self, new_content: String) {
        self.contents = new_content;
    }

    pub fn change_category(&mut self, new_category_id: i64) {
        self.category_id = new_category_id;
    }

    pub fn change_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    pub fn get_id(&self) -> Option<Uuid> {
        self.id
    }

    pub fn get_category_id(&self) -> i64 {
        self.category_id
    }

    pub fn get_member_id(&self) -> Uuid {
        self.member_id
    }

    pub fn get_title(&self) -> String {
        self.title.to_owned()
    }

    pub fn get_contents(&self) -> String {
        self.contents.to_owned()
    }

    pub fn get_created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn get_updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }

    pub fn get_view_count(&self) -> i64 {
        self.view_count
    }
}
