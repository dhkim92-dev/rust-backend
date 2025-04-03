pub mod MemberMapper {
    use crate::domain::{member::entity::MemberEntity, member::schema::Model as MemberModel};

    pub fn to_domain(ormEntity: &MemberModel) -> MemberEntity {
        MemberEntity {
            id: Some(ormEntity.id),
            nickname: ormEntity.nickname.clone(),
            email: ormEntity.email.clone(),
            password: ormEntity.password.clone(),
            role: ormEntity.role.clone(),
            created_at: ormEntity.created_at.clone(),
            updated_at: ormEntity.updated_at.clone(),
            is_activated: ormEntity.is_activated,
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
