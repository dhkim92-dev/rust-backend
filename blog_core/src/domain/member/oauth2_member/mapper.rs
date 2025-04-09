pub mod oauth2_member_mapper {
    
    use sea_orm::ActiveValue::{NotSet, Set};

    use crate::domain::member::oauth2_member::entity::OAuth2MemberEntity;
    use crate::domain::member::oauth2_member::schema::{Model, ActiveModel};

    pub fn to_domain(orm_model: Model) -> OAuth2MemberEntity {
        OAuth2MemberEntity::new(
            orm_model.id,
            orm_model.provider,
            orm_model.user_id,
            orm_model.member_id,
            orm_model.email,
            orm_model.access_token,
        )
    }

    pub fn to_orm(entity: OAuth2MemberEntity) -> ActiveModel {
        ActiveModel {
            id: if entity.get_id().is_none() {
                NotSet
            } else {
                Set(entity.get_id())
            },
            provider: Set(entity.get_provider().clone()),
            user_id: Set(entity.get_user_id().clone()),
            member_id: Set(entity.get_member_id()),
            email: Set(entity.get_email().clone()),
            access_token: Set(entity.get_access_token().clone()),
        }
    }
}
