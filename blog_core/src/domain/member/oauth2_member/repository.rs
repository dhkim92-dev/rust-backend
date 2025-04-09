use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};
use shaku::{Component, Interface};
use crate::common::error_code::ErrorCode;
use crate::common::AppError;
use super::entity::OAuth2MemberEntity;
use super::mapper::oauth2_member_mapper;
use super::schema::{Entity as OAuth2Member, Column};

#[derive(Component)]
#[shaku(interface = LoadOAuth2MemberPort)]
pub struct SeaormLoadOAuth2MemberAdapter {}

#[async_trait::async_trait]
impl LoadOAuth2MemberPort for SeaormLoadOAuth2MemberAdapter {

    async fn find_by_provider_and_user_id(
        &self, 
        txn: &DatabaseTransaction,
        provider: String,
        user_id: String
    ) -> Option<OAuth2MemberEntity> {
        let result = OAuth2Member::find()
            .filter(Column::Provider.eq(provider))
            .filter(Column::UserId.eq(user_id))
            .one(txn)
            .await
            .map_err(|e| AppError::from(ErrorCode::DbError))
            .ok()?;

        Ok(oauth2_member_mapper::to_domain(result))
    }
}

#[derive(Component)]
#[shaku(interface = SaveOAuth2MemberPort)]
pub struct SeaormSaveOAuth2MemberAdapter {}

#[async_trait::async_trait]
impl SaveOAuth2MemberPort for SeaormSaveOAuth2MemberAdapter {

    async fn save(&self, txn: &DatabaseTransaction, entity: OAuth2MemberEntity) -> Result<OAuth2MemberEntity, AppError> {

        let active_model = oauth2_member_mapper::to_orm(entity);
        
        if active_model.id.is_set() {
            return Err(AppError::with_message(ErrorCode::InternalServerError, "Save 메서드는 신규 생성에만 사용해야 합니다."));
        }

        active_model.id = Set(uuid::Uuid::new_v4());

        let result = active_model.insert(txn)
            .await.map_err(|e| AppError::from(ErrorCode::DbError))?;
        let entity = oauth2_member_mapper::to_domain(result);

        Ok(entity)
    }
}



#[async_trait::async_trait]
pub trait LoadOAuth2MemberPort: Interface {

    async fn find_by_provider_and_user_id(
        &self, 
        txn: &DatabaseTransaction,
        provider: String,
        user_id: String
    ) -> Option<OAuth2MemberEntity>;
}

#[async_trait::async_trait]
pub trait SaveOAuth2MemberPort: Interface {

    async fn save(&self, txn: &DatabaseTransaction, entity: OAuth2MemberEntity) -> Result<OAuth2MemberEntity, AppError>;
}
