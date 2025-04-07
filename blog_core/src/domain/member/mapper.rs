pub mod member_mapper {
    use crate::domain::{member::entity::MemberEntity, member::schema::Model as MemberModel};

    pub fn to_domain(orm_entity: &MemberModel) -> MemberEntity {
        MemberEntity {
            id: Some(orm_entity.id),
            nickname: orm_entity.nickname.clone(),
            email: orm_entity.email.clone(),
            password: orm_entity.password.clone(),
            role: orm_entity.role.clone(),
            created_at: orm_entity.created_at.clone(),
            updated_at: orm_entity.updated_at.clone(),
            is_activated: orm_entity.is_activated,
        }
    }

    pub fn to_orm(domain_entity: &MemberEntity) -> MemberModel {
        MemberModel {
            id: domain_entity.id.unwrap(),
            nickname: domain_entity.nickname.clone(),
            email: domain_entity.email.clone(),
            password: domain_entity.password.clone(),
            role: domain_entity.role.clone(),
            created_at: domain_entity.created_at.clone(),
            updated_at: domain_entity.updated_at.clone(),
            is_activated: domain_entity.is_activated,
        }
    }
}
