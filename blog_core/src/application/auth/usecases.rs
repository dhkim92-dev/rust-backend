use crate::common::error::error_code::ErrorCode;
use crate::common::AppError;
use shaku::Interface;

pub struct LoginCommand {
    pub principal: String,
    pub credential: String,
}

#[derive(Debug)]
pub struct LoginCommandResult {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct OAuth2LoginCommand {
    pub provider: String,
    pub user_id: String,
    pub email: Option<String>,
    pub access_token: String,
}

#[derive(Debug)]
pub struct JwtReissueResult {
    pub access_token: String,
}

#[async_trait::async_trait]
pub trait LoginUseCase: Interface {
    async fn login(&self, command: LoginCommand) -> Result<LoginCommandResult, ErrorCode>;

    async fn login_by_oauth2(&self, command: OAuth2LoginCommand) -> Result<LoginCommandResult, AppError>;
}

#[async_trait::async_trait]
pub trait JwtUseCase: Interface {
    async fn refresh_jwt(&self, refresh_token: String) -> Result<JwtReissueResult, ErrorCode>;
}
