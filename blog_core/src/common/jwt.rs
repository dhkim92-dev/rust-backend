use std::sync::Arc;

use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
use shaku::{Interface, Component};
use crate::config::{AppConfig, ConfigProvider};
use crate::domain::member::entity::MemberEntity;

use super::error::error_code::ErrorCode;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: String,
    pub email: String,
    pub nickname: String,
    pub is_activated: bool,
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenClaims {
    sub: String,
    exp: usize,
    iat: usize,
    iss: String,
    aud: String,
}

fn map_to_access_token_claims(cfg: Arc<AppConfig>, member: &MemberEntity)-> AccessTokenClaims {
    AccessTokenClaims {
        sub: member.id.unwrap().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::seconds(3600)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
        iss: cfg.jwt_issuer.to_string(),
        aud: cfg.jwt_audience.to_string(),
        email: member.email.clone(),
        nickname: member.nickname.clone(),
        roles: vec!(format!("{}_{}", "ROLE", member.role.to_string())),
        is_activated: member.is_activated,
    }
}

fn map_to_refresh_token_claims(cfg: Arc<AppConfig>, member: &MemberEntity) -> RefreshTokenClaims {
    RefreshTokenClaims {
        sub: member.id.unwrap().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::seconds(3600)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
        iss: cfg.jwt_issuer.to_string(),
        aud: cfg.jwt_audience.to_string(),
    }
}

pub trait JwtService: Interface {
    fn create_access_token(&self, member: &MemberEntity) -> Result<String, ErrorCode>;
    fn create_refresh_token(&self, member: &MemberEntity) -> Result<String, ErrorCode>;
    fn decode_access_token(&self, token: &str) -> Result<AccessTokenClaims, ErrorCode>;
    fn decode_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims, ErrorCode>;
}

#[derive(Component)]
#[shaku(interface = JwtService)]
pub struct JwtServiceImpl {
    #[shaku(inject)]
    config: Arc<dyn ConfigProvider>,
}

impl JwtService for JwtServiceImpl {

    fn create_access_token(&self, member: &MemberEntity) -> Result<String, ErrorCode> {
        let claims = map_to_access_token_claims(self.config.get(), member);
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let secret = &self.config.get().jwt_access_token_secret;
        encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()))
            .map_err(|_| ErrorCode::JWT_BUILD_CLAIMS_EXCEPTION)
    }

    fn create_refresh_token(&self, member: &MemberEntity) -> Result<String, ErrorCode> {
        let claims = map_to_refresh_token_claims(self.config.get(), member);
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let secret = &self.config.get().jwt_refresh_token_secret;
        encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()))
            .map_err(|_| ErrorCode::JWT_BUILD_CLAIMS_EXCEPTION)
    }

    fn decode_access_token(&self, token: &str) -> Result<AccessTokenClaims, ErrorCode> {
        let secret = &self.config.get().jwt_access_token_secret;
        let token_data = jsonwebtoken::decode::<AccessTokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        ).map_err(|_| ErrorCode::INVALID_JWT_TOKEN);
        
        Ok(token_data.unwrap().claims)
    }

    fn decode_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims, ErrorCode> {
        let secret = &self.config.get().jwt_refresh_token_secret;
        let token_data = jsonwebtoken::decode::<RefreshTokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        ).map_err(|_| ErrorCode::INVALID_JWT_TOKEN);
        Ok(token_data.unwrap().claims)
    }
}

