use crate::common::error_code::ErrorCode;
use crate::common::AppError;
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct BoardEntity {
    board_id: Option<u64>,
    name: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

/* public methods */
impl BoardEntity {

    pub fn new(
        board_id: Option<u64>,
        name: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        BoardEntity {
            board_id,
            name,
            created_at: created_at.unwrap_or_else(|| Utc::now().naive_utc()),
            updated_at,
        }       
    }

    fn validate(&self) -> Result<bool, AppError> {
        if self.name.len() < 1 || self.name.len() > 15 {
            return Err( AppError::with_message(ErrorCode::ValidationError, "이름은 2자 이상 14자 이하여야합니다.") );
        }

        Ok(true)
    }

    pub fn get_created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn get_updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at 
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_id(&self) -> Option<u64> {
        self.board_id
    }

    pub fn change_board_name(&mut self, name: &str) -> Result<(), AppError> {
        if name.len() < 1 || name.len() > 15 {
            return Err(AppError::with_message(ErrorCode::ValidationError, "이름은 2자 이상 14자 이하여야합니다."));
        }
        self.name = name.to_owned();
        Ok(())
    }

    pub fn update(&mut self) -> Result<bool, AppError> {
        if self.validate().is_ok() {
            // Update logic here
            self.updated_at = Some(Utc::now().naive_utc());
            Ok(true)
        } else {
            Err(AppError::with_message(ErrorCode::ValidationError, "Invalid board name"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::BoardEntity;

    #[test]
    fn validate_board_test() {
        let mut board = BoardEntity::new(None, "test".to_string(), None, None);
        assert_eq!(board.validate().is_ok(), true);
        let err = board.change_board_name("").err().unwrap();
        assert_eq!(err.message, "이름은 2자 이상 14자 이하여야합니다.");
    }

    #[test]
    fn change_name_test() {
        let mut board = BoardEntity::new(None, "test".to_string(), None, None);
        assert_eq!(board.get_name(), "test".to_string());
        board.change_board_name("test2");
        assert_eq!(board.get_name(), "test2".to_string());
    }
}

