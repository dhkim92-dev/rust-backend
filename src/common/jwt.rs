use std::sync::Arc;

use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
use shaku::{Interface, Component};
use crate::config::{AppConfig, ConfigProvider};
use crate::domain::member::entity::MemberEntity;

#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenClaims {
    sub: String,
    exp: usize,
    iat: usize,
    iss: String,
    aud: String,
    email: String,
    nickname: String,
    is_activated: bool,
    roles: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RefreshTokenClaims {
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
    fn create_access_token(&self, member: &MemberEntity) -> String;
    fn create_refresh_token(&self, member: &MemberEntity) -> String;
    fn decode_access_token(&self, token: &str) -> Result<AccessTokenClaims, String>;
    fn decode_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims, String>;
}

#[derive(Component)]
#[shaku(interface = JwtService)]
pub struct JwtServiceImpl {
    #[shaku(inject)]
    config: Arc<dyn ConfigProvider>,
}

impl JwtService for JwtServiceImpl {

    fn create_access_token(&self, member: &MemberEntity) -> String {
        let claims = map_to_access_token_claims(self.config.get(), member);
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let secret = &self.config.get().jwt_access_token_secret;
        encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()))
            .map_err(|_| "Failed to create access token".to_string())
            .unwrap_or_else(|_| "Failed to create access token".to_string())
    }

    fn create_refresh_token(&self, member: &MemberEntity) -> String {
        let claims = map_to_refresh_token_claims(self.config.get(), member);
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let secret = &self.config.get().jwt_refresh_token_secret;
        encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()))
            .map_err(|_| "Failed to create refresh token".to_string())
            .unwrap_or_else(|_| "Failed to create refresh token".to_string())
    }

    fn decode_access_token(&self, token: &str) -> Result<AccessTokenClaims, String> {
        let secret = &self.config.get().jwt_access_token_secret;
        let token_data = jsonwebtoken::decode::<AccessTokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| "Failed to decode access token".to_string())?;
        Ok(token_data.claims)
    }

    fn decode_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims, String> {
        let secret = &self.config.get().jwt_refresh_token_secret;
        let token_data = jsonwebtoken::decode::<RefreshTokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| "Failed to decode refresh token".to_string())?;
        Ok(token_data.claims)
    }
}

