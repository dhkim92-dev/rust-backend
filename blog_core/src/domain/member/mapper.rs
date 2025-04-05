pub mod MemberMapper {
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

    pub fn to_orm(domainEntity: &MemberEntity) -> MemberModel {
        MemberModel {
            id: domainEntity.id.unwrap(),
            nickname: domainEntity.nickname.clone(),
            email: domainEntity.email.clone(),
            password: domainEntity.password.clone(),
            role: domainEntity.role.clone(),
            created_at: domainEntity.created_at.clone(),
            updated_at: domainEntity.updated_at.clone(),
            is_activated: domainEntity.is_activated,
        }
    }
}
