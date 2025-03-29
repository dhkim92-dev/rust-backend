use sea_orm::prelude::async_trait::async_trait;

use crate::application::dto::auth::{LoginCommand, LoginCommandResponse};
use crate::domain::member::repository::LoadMemberPort;
use std::sync::{Arc};


#[async_trait]
pub trait AuthUsecase{
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

        Ok(LoginCommandResponse{
            typ: "Bearer".to_string(),
            access_token: "1234".to_string(),
            refresh_token: "5678".to_string()
        })
    }
} 
