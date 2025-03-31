use super::usecases::{LoginUseCase, LoginCommand, LoginCommandResult};
use crate::domain::member::repository::LoadMemberPort;
use crate::common::error::error_code::ErrorCode as E;
use shaku::Component;
use std::sync::Arc;
#[derive(Component)]
#[shaku(interface = LoginUseCase)]
pub struct AuthService {
    #[shaku(inject)]
    member_repository: Arc<dyn LoadMemberPort>
}

#[async_trait::async_trait]
impl LoginUseCase for AuthService {
    async fn login(&self, command: LoginCommand) -> Result<LoginCommandResult, E> {
        let member = self.member_repository.find_by_email(&command.principal).await.map_err(|_| {
            E::with_message(E::EMAIL_PASSWORD_MISMATCH, "DB error")
        })?
        .ok_or(E::MEMBER_NOT_FOUND)?;

        let valid_password = bcrypt::verify(&command.credential.as_bytes(), &member.password).map_err(|_| {
            E::EMAIL_PASSWORD_MISMATCH
        })?;

        println!("valid_password: {}", valid_password);

        if !valid_password {
            return Err(E::EMAIL_PASSWORD_MISMATCH);
        }

        Ok(LoginCommandResult {
            access_token: member.id.unwrap().to_string(),
            refresh_token: member.password.to_string()
        })
    }
}
