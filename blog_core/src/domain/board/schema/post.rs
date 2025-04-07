use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveModelBehavior;
use crate::domain::board;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "article")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: uuid::Uuid,
    pub member_id: Uuid,
    pub category_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub contents: String,
    pub view_count: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::board::Entity",
        from = "Column::CategoryId",
        to = "super::board::Column::Id"
    )]
    Board,
    #[sea_orm(
        belongs_to = "crate::domain::member::schema::Entity",
        from = "Column::MemberId",
        to = "crate::domain::member::schema::Column::Id"
    )]
    Member
}

impl Related<board::schema::board::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Board.def()
    }
}

impl Related<crate::domain::member::schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
