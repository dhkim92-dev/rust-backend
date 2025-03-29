use bcrypt::bcrypt;
use sea_orm::prelude::async_trait::async_trait;
use crate::application::dto::auth::{LoginCommand, LoginCommandResponse};
use crate::common::jwt::{create_access_token, create_refresh_token};
use crate::domain::member::repository::LoadMemberPort;
use crate::common::error::error_code::ErrorCode;
use crate::common::error::member_error::MemberError;
use crate::common::error::auth_error::AuthError;
use crate::config::AppContext;
use crate::domain::member::entity::MemberEntity;
use std::sync::{Arc};


#[async_trait]
pub trait AuthUsecase: Send + Sync {
    async fn login_with_email_password(&self, command: LoginCommand) -> Result<LoginCommandResponse, Box<dyn ErrorCode>>;
}

pub struct AuthService {
    context: Arc<AppContext>,
    member_repository: Arc<dyn LoadMemberPort>,
}

impl AuthService {

    pub fn new(context: Arc<AppContext>, member_repository: Arc<dyn LoadMemberPort>) -> Self {
        Self { context, member_repository }
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

        let access_token = create_access_token(self.context.clone(), &member);
        let refresh_token = create_refresh_token(self.context.clone(), &member);


        Ok(LoginCommandResponse{
            typ: "Bearer".to_string(),
            access_token: access_token,
            refresh_token: refresh_token
        })
    }
} 
