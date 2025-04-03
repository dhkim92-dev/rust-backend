use super::usecases::{
    JwtReissueResult, JwtUseCase, LoginCommand, LoginCommandResult, LoginUseCase,
};
use crate::common::database::DbConnProvider;
use crate::common::error::error_code::ErrorCode as E;
use crate::common::jwt::JwtService;
use crate::domain::member::repository::LoadMemberPort;
use uuid::Uuid;
use shaku::Component;
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = LoginUseCase)]
pub struct AuthService {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_member_port: Arc<dyn LoadMemberPort>,
    #[shaku(inject)]
    jwt_service: Arc<dyn JwtService>,
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
impl LoginUseCase for AuthService {
    async fn login(&self, command: LoginCommand) -> Result<LoginCommandResult, E> {
        let txn = self
            .db
            .ro_txn()
            .await
            .map_err(|_| E::InternalServerError)?;
        let member = self
            .load_member_port
            .find_by_email(&txn, &command.principal)
            .await;

        txn.commit().await;

        let member = match member {
            Ok(Some(member)) => member,
            Ok(None) => return Err(E::MemberNotFound),
            Err(_) => return Err(E::InternalServerError),
        };

        let valid_password = bcrypt::verify(&command.credential.as_bytes(), &member.password)
            .map_err(|_| E::EmailPasswordMismatch)?;

        if !valid_password {
            return Err(E::EmailPasswordMismatch);
        }

        Ok(LoginCommandResult {
            access_token: self
                .jwt_service
                .create_access_token(&member)
                .map_err(|_| E::Unauthorized)?,
            refresh_token: self
                .jwt_service
                .create_refresh_token(&member)
                .map_err(|_| E::Unauthorized)?,
        })
    }
}

#[async_trait::async_trait]
impl JwtUseCase for JwtUseCaseImpl {
    async fn refresh_jwt(&self, refresh_token: String) -> Result<JwtReissueResult, E> {
        let txn = self.db.ro_txn().await?;

        let member_id = self
            .jwt_service
            .decode_refresh_token(&refresh_token)
            .map_err(|_| E::Unauthorized)
            .and_then(|claims| Uuid::parse_str(claims.sub.as_str()).map_err(|_| E::Unauthorized))?;

        let member = match self.load_member_port.find_by_id(&txn, member_id).await? {
            Some(member) => member,
            None => return Err(E::NotFound),
        };

        txn.commit().await;

        Ok(JwtReissueResult {
            access_token: self
                .jwt_service
                .create_access_token(&member)
                .map_err(|_| E::Unauthorized)?,
        })
    }
}
