use super::usecases::{LoginUseCase, LoginCommand, LoginCommandResult};
use crate::common::middleware::transaction::get_transaction;
use crate::domain::member::repository::LoadMemberPort;
use crate::common::error::error_code::ErrorCode as E;
use crate::common::jwt::JwtService;

use shaku::Component;
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = LoginUseCase)]
pub struct AuthService {
    #[shaku(inject)]
    member_repository: Arc<dyn LoadMemberPort>,
    #[shaku(inject)]
    jwt_service: Arc<dyn JwtService>,
}

#[async_trait::async_trait]
impl LoginUseCase for AuthService {

    async fn login(&self, command: LoginCommand) -> Result<LoginCommandResult, E> {
        let txn = get_transaction().await.ok_or(E::INTERNAL_SERVER_ERROR)?;

        let member = self.member_repository.find_by_email(&txn, &command.principal).await.map_err(|_| {
            E::with_message(E::EMAIL_PASSWORD_MISMATCH, "DB error")
        })?
        .ok_or(E::MEMBER_NOT_FOUND)?;

        let valid_password = bcrypt::verify(&command.credential.as_bytes(), &member.password).map_err(|_| {
            E::EMAIL_PASSWORD_MISMATCH
        })?;

        if !valid_password {
            return Err(E::EMAIL_PASSWORD_MISMATCH);
        }

        Ok(LoginCommandResult {
            access_token: self.jwt_service.create_access_token(&member).map_err(|_| {
                E::UNAUTHORIZED
            })?,
            refresh_token: self.jwt_service.create_refresh_token(&member).map_err(|_| {
                E::UNAUTHORIZED
            })?,
        })
    }
}
