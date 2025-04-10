use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, Related, RelationDef, RelationTrait};
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "oauth2_member")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub provider: String,
    pub user_id: String,
    pub member_id: Uuid,
    pub email: Option<String>,
    pub access_token: String,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::member::schema::Entity",
        from = "Column::MemberId",
        to = "crate::domain::member::schema::Column::Id",
    )]
    Member,
}

impl Related<crate::domain::member::schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
