use sea_orm::*;
use crate::domain::member::{self, Entity as Member};

pub struct SeaOrmMemberRepository {
    ctx: axum::Extension<crate::config::AppContext>,
}

impl SeaOrmMemberRepository {

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<member::Model>, sea_orm::DbErr> {
        Member::find_by_id(id)
            .one(&self.ctx.db)
            .await
    }

    pub  async fn  find_by_email(&self,  email:  &String) ->  Result<Option<member::Model>, sea_orm::DbErr> {
        Member::find()
            .filter(member::Column::Email.eq(email))
            .one(&self.ctx.db)
            .await
    }
} 
