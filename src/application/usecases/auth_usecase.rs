use bcrypt::bcrypt;
use sea_orm::prelude::async_trait::async_trait;

use crate::application::dto::auth::{LoginCommand, LoginCommandResponse};
use crate::domain::member::repository::LoadMemberPort;
use std::sync::{Arc};


#[async_trait]
pub trait AuthUsecase: Send + Sync {
    async fn login_with_email_password(&self, command: LoginCommand) -> anyhow::Result<LoginCommandResponse>;
}

pub struct AuthService {
    member_repository: Arc<dyn LoadMemberPort>,
}

impl AuthService {

    pub fn new(member_repository: Arc<dyn LoadMemberPort>) -> Self {
        Self { member_repository }
    }
}

#[async_trait]
impl AuthUsecase for AuthService {

    async fn login_with_email_password(&self, command: LoginCommand) -> anyhow::Result<LoginCommandResponse> {
        let member = self.member_repository.find_by_email(&command.principal).await
            .map_err(|_| anyhow::anyhow!("Member not found"))?
            .ok_or_else(|| anyhow::anyhow!("Member not found"))?;

        if !bcrypt::verify(&command.credential, &member.password)? {
            return Err(anyhow::anyhow!("Invalid password"))?;
        }


        Ok(LoginCommandResponse{
            typ: "Bearer".to_string(),
            access_token: member.email,
            refresh_token: "1234".to_string()
        })
    }
} 
