use super::usecases::{
    JwtReissueResult, JwtUseCase, LoginCommand, LoginCommandResult, LoginUseCase, OAuth2LoginCommand,
};
//use crate::application::member::{MemberCreateCommand, MemberCreateUseCase};
use crate::application::oauth2::generate_rand;
use crate::common::database::DbConnProvider;
use crate::common::error::error_code::ErrorCode;
use crate::common::jwt::JwtService;
use crate::common::AppError;
use crate::domain::member::entity::MemberEntity;
use crate::domain::member::oauth2_member::entity::OAuth2MemberEntity;
use crate::domain::member::oauth2_member::repository::{LoadOAuth2MemberPort, SaveOAuth2MemberPort};
use crate::domain::member::repository::{LoadMemberPort, SaveMemberPort};
use sea_orm::DatabaseTransaction;
use shaku::Component;
use std::process::exit;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = LoginUseCase)]
pub struct AuthService {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_member_port: Arc<dyn LoadMemberPort>,
    #[shaku(inject)]
    save_member_port: Arc<dyn SaveMemberPort>,
    #[shaku(inject)]
    jwt_service: Arc<dyn JwtService>,
    #[shaku(inject)]
    load_oauth2_member_port: Arc<dyn LoadOAuth2MemberPort>,
    #[shaku(inject)]
    save_oauth2_member_port: Arc<dyn SaveOAuth2MemberPort>,
}

impl AuthService {

    fn create_new_member(&self, email: String, nickname: String) -> MemberEntity {
        let entity = MemberEntity {
            id: None,
            email,
            nickname,
            password: bcrypt::hash(generate_rand(32), 10).unwrap(),
            role: "MEMBER".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
            is_activated: true,
        };

        tracing::debug!("create_new_member: {:?}", entity);

        entity
    }

    async fn do_join_or_union(&self, 
        txn: &DatabaseTransaction, 
        command: OAuth2LoginCommand
    ) -> Result<MemberEntity, AppError> {

        let virtual_email_addr = format!("{}-{}@dohoon-kim.kr", command.provider.clone(), command.user_id.clone());
        
        let member = match command.email.clone() {
            Some(email) => {
                let exist_member = self.load_member_port
                    .find_by_email(txn, &email)
                    .await?;

                let exist_member = match exist_member {
                    Some(member) => {
                        member
                    },
                    None => {
                        let new_member = self.create_new_member(
                            format!("{}-{}@dohoon-kim.kr", command.provider, command.user_id).to_string(), 
                            format!("{}:{}", command.provider, command.user_id).to_string()
                        );

                        self.save_member_port
                            .save(txn, new_member)
                            .await?
                    }
                };
                tracing::debug!("exist_member: {:?}", exist_member);
                exist_member
            },
            None => {
                let new_member = self.create_new_member(
                    format!("{}-{}@dohoon-kim.kr", command.provider, command.user_id).to_string(), 
                    format!("{}:{}", command.provider, command.user_id).to_string()
                );
                self.save_member_port
                    .save(txn, new_member)
                    .await?
            }
        };

        let oauth2_member = OAuth2MemberEntity::new(
            None,
            member.id.unwrap(),
            command.provider,
            command.user_id,
            Some(member.email.clone()),
            command.access_token,
        );

        self.save_oauth2_member_port
            .save(txn, oauth2_member)
            .await?;

        Ok(member)
    }
}


#[async_trait::async_trait]
impl LoginUseCase for AuthService {

    async fn login_by_oauth2(
        &self,
        command: OAuth2LoginCommand,
    ) -> Result<LoginCommandResult, AppError> {
        let txn = self.db.rw_txn().await.map_err(|_| ErrorCode::InternalServerError)?;

        let oauth2_member = self
            .load_oauth2_member_port
            .find_by_provider_and_user_id(&txn, command.provider.clone(), command.user_id.clone())
            .await;

        let member = match oauth2_member {
            Some(oauth2_member) => {
                self.load_member_port.find_by_id(&txn, oauth2_member.get_member_id()).await?
            },
            None => {
                Some(self.do_join_or_union(&txn, command.clone()).await?)
            }
        }.unwrap();

        txn.commit().await?;

        Ok(LoginCommandResult {
            access_token: self
                .jwt_service
                .create_access_token(&member)
                .map_err(|_| ErrorCode::Unauthorized)?,
            refresh_token: self
                .jwt_service
                .create_refresh_token(&member)
                .map_err(|_| ErrorCode::Unauthorized)?,
        })
    }

    async fn login(&self, command: LoginCommand) -> Result<LoginCommandResult, ErrorCode> {
        let txn = self.db.ro_txn().await.map_err(|_| ErrorCode::InternalServerError)?;
        let member = self
            .load_member_port
            .find_by_email(&txn, &command.principal)
            .await;

        txn.commit().await?;

        let member = match member {
            Ok(Some(member)) => member,
            Ok(None) => return Err(ErrorCode::MemberNotFound),
            Err(_) => return Err(ErrorCode::InternalServerError),
        };

        let valid_password = bcrypt::verify(&command.credential.as_bytes(), &member.password)
            .map_err(|_| ErrorCode::EmailPasswordMismatch)?;

        if !valid_password {
            return Err(ErrorCode::EmailPasswordMismatch);
        }

        Ok(LoginCommandResult {
            access_token: self
                .jwt_service
                .create_access_token(&member)
                .map_err(|_| ErrorCode::Unauthorized)?,
            refresh_token: self
                .jwt_service
                .create_refresh_token(&member)
                .map_err(|_| ErrorCode::Unauthorized)?,
        })
    }
}

#[derive(Component)]
#[shaku(interface = JwtUseCase)]
pub struct JwtUseCaseImpl {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_member_port: Arc<dyn LoadMemberPort>,
    #[shaku(inject)]
    jwt_service: Arc<dyn JwtService>,
}


#[async_trait::async_trait]
impl JwtUseCase for JwtUseCaseImpl {
    async fn refresh_jwt(&self, refresh_token: String) -> Result<JwtReissueResult, ErrorCode> {
        let txn = self.db.ro_txn().await?;

        let member_id = self
            .jwt_service
            .decode_refresh_token(&refresh_token)
            .map_err(|_| ErrorCode::Unauthorized)
            .and_then(|claims| Uuid::parse_str(claims.sub.as_str()).map_err(|_| ErrorCode::Unauthorized))?;

        let member = match self.load_member_port.find_by_id(&txn, member_id).await? {
            Some(member) => member,
            None => return Err(ErrorCode::NotFound),
        };

        txn.commit().await?;

        Ok(JwtReissueResult {
            access_token: self
                .jwt_service
                .create_access_token(&member)
                .map_err(|_| ErrorCode::Unauthorized)?,
        })
    }
}
