use bcrypt::bcrypt;
use sea_orm::prelude::async_trait::async_trait;
use crate::application::dto::auth::{LoginCommand, LoginCommandResponse};
use crate::domain::member::repository::LoadMemberPort;
use crate::common::error::error_code::ErrorCode;
use crate::common::error::member_error::MemberError;
use crate::common::error::auth_error::AuthError;
use std::sync::{Arc};


pub fn create_access_token(config: AppContext, member: &Member) -> String {

}

pub fn create_refresh_token(config: AppContext, member: &Member) -> String {
}


#[async_trait]
pub trait AuthUsecase: Send + Sync {
    async fn login_with_email_password(&self, command: LoginCommand) -> Result<LoginCommandResponse, Box<dyn ErrorCode>>;
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

    async fn login_with_email_password(&self, command: LoginCommand) 
        -> Result<LoginCommandResponse, Box<dyn ErrorCode>> {

        let member = self.member_repository.find_by_email(&command.principal)
            .await
            .map_err(|_| Box::new(MemberError::MemberNotExist) as Box<dyn ErrorCode>)?
            .ok_or_else(|| Box::new(MemberError::MemberNotExist) as Box<dyn ErrorCode>)?;

    let is_valid_password = bcrypt::verify(&command.credential, &member.password)
        .map_err(|_| Box::new(AuthError::EmailPasswordMismatch) as Box<dyn ErrorCode>)?;

        if !is_valid_password {
            return Err(Box::new(AuthError::EmailPasswordMismatch) as Box<dyn ErrorCode>);
        }


        Ok(LoginCommandResponse{
            typ: "Bearer".to_string(),
            access_token: member.email,
            refresh_token: "1234".to_string()
        })
    }
} 
