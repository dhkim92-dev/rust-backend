use super::usecases::{LoginCommand, LoginCommandResult, LoginUseCase};
// use crate::common::middleware::transaction::get_transaction;
use crate::common::database::DbConnProvider;
use crate::common::error::error_code::ErrorCode as E;
use crate::common::jwt::JwtService;
use crate::domain::member::repository::LoadMemberPort;
use sea_orm::TransactionTrait;

use shaku::Component;
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = LoginUseCase)]
pub struct AuthService {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    member_repository: Arc<dyn LoadMemberPort>,
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
            .map_err(|_| E::INTERNAL_SERVER_ERROR)?;
        let member = self
            .member_repository
            .find_by_email(&txn, &command.principal)
            .await;

        txn.commit().await;

        let member = match member {
            Ok(Some(member)) => member,
            Ok(None) => return Err(E::NOT_FOUND),
            Err(_) => return Err(E::INTERNAL_SERVER_ERROR),
        };

        let valid_password = bcrypt::verify(&command.credential.as_bytes(), &member.password)
            .map_err(|_| E::EMAIL_PASSWORD_MISMATCH)?;

        if !valid_password {
            return Err(E::EMAIL_PASSWORD_MISMATCH);
        }

        Ok(LoginCommandResult {
            access_token: self
                .jwt_service
                .create_access_token(&member)
                .map_err(|_| E::UNAUTHORIZED)?,
            refresh_token: self
                .jwt_service
                .create_refresh_token(&member)
                .map_err(|_| E::UNAUTHORIZED)?,
        })
    }
}
