use crate::common::error::error_code::ErrorCode;
use shaku::Interface;

pub struct LoginCommand {
    pub principal: String,
    pub credential: String
}

pub struct LoginCommandResult {
    pub access_token: String,
    pub refresh_token: String
}

#[async_trait::async_trait]
pub trait LoginUseCase : Interface {
    async fn login(&self, command: LoginCommand) -> Result<LoginCommandResult, ErrorCode>;
}
