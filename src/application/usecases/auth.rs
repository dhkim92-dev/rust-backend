use crate::application::dto::auth::{LoginCommand, LoginCommandResponse};
use crate::domain::member_repository::SeaOrmMemberRepository;
use crate::domain::member;

pub trait AuthUsecase{
    fn new(member_repository: std::sync::Arc<SeaOrmMemberRepository>) -> Self;
    pub async fn login_with_email_password(&self, command: LoginCommand) -> anyhow::Result<LoginCommandResponse>;
}

pub struct AuthService {
    member_repository: std::sync::Arc<SeaOrmMemberRepository>,
}

impl AuthUsecase for AuthService {

    fn new(member_repository: std::sync::Arc<SeaOrmMemberRepository>) -> Self {
        Self { member_repository }
    }

    async fn login_with_email_password(&self, command: LoginCommand) -> anyhow::Result<LoginCommandResponse> {
        let member: member::Model 
            = self.member_repository.find_by_email(&command.principal).await.expect("Member not found")
            .unwrap();

        Ok(LoginCommandResponse{
            typ: "Bearer".to_string(),
            access_token: member.id.clone().to_string(),
            refresh_token: member.email.clone().to_string()
        })
    }
}
