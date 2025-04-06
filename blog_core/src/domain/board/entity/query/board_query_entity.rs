use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;

#[derive(Debug, Clone, PartialEq, Eq, FromQueryResult)]
pub struct QBoardEntity {
    pub id: i64,
    pub name: String,
    pub count: i64,
}
