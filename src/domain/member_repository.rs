use sea_orm::*;
use crate::domain::member::{self, Entity as Member};

struct SeaOrmMemberRepository {
    db: DatabaseConnection,
}


impl SeaOrmMemberRepository {

    /* pub async fn persist(&self, member: &Member) -> Result<Member, sea_orm::DbErr> {

        let mut active_model = member::ActiveModel {
            id: Set(member.id),
            nickname: Set(member.nickname.clone()),
            email: Set(member.email.clone()),
            password: Set(member.password.clone()),
            role: Set(member.role.clone()),
            created_at: Set(member.created_at),
            updated_at: Set(member.updated_at),
        };

        active_model.insert(&self.db).await
    } */

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<member::Model>, sea_orm::DbErr> {
        Member::find_by_id(id)
            .one(&self.db)
            .await
    }

    pub  async fn  find_by_email(&self,  email:  &String) ->  Result<Option<member::Model>, sea_orm::DbErr> {
        Member::find()
            .filter(member::Column::Email.eq(email))
            .one(&self.db)
            .await
    }
} 
