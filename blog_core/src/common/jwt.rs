use std::sync::Arc;

use super::error::error_code::ErrorCode;
use crate::config::{AppConfig, ConfigProvider};
use crate::domain::member::entity::MemberEntity;
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use tracing::error;

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

impl From<MemberEntity> for AccessTokenClaims {
    fn from(member: MemberEntity) -> Self {
        AccessTokenClaims {
            sub: member.id.unwrap().to_string(),
            exp: 0,
            iat: 0,
            iss: "".to_string(),
            aud: "".to_string(),
            email: member.email.clone(),
            nickname: member.nickname.clone(),
            is_activated: member.is_activated,
            roles: vec![format!("{}_{}", "ROLE", member.role.to_string())],
        }
    }
}

impl Into<AccessTokenClaims> for String {
    fn into(self) -> AccessTokenClaims {
        AccessTokenClaims {
            sub: self,
            exp: 0,
            iat: 0,
            iss: "".to_string(),
            aud: "".to_string(),
            email: "".to_string(),
            nickname: "".to_string(),
            is_activated: false,
            roles: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: String,
}

fn map_to_access_token_claims(cfg: Arc<AppConfig>, member: &MemberEntity) -> AccessTokenClaims {
    AccessTokenClaims {
        sub: member.id.unwrap().to_string(),
        exp: (chrono::Utc::now()
            + chrono::Duration::milliseconds(cfg.jwt_access_token_expire as i64))
        .timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
        iss: cfg.jwt_issuer.to_string(),
        aud: cfg.jwt_audience.to_string(),
        email: member.email.clone(),
        nickname: member.nickname.clone(),
        roles: vec![format!("{}_{}", "ROLE", member.role.to_string())],
        is_activated: member.is_activated,
    }
}

fn map_to_refresh_token_claims(cfg: Arc<AppConfig>, member: &MemberEntity) -> RefreshTokenClaims {
    RefreshTokenClaims {
        sub: member.id.unwrap().to_string(),
        exp: (chrono::Utc::now()
            + chrono::Duration::milliseconds(cfg.jwt_refresh_token_expire as i64))
        .timestamp() as usize,
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
        encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|_| ErrorCode::JWT_BUILD_CLAIMS_EXCEPTION)
    }

    fn create_refresh_token(&self, member: &MemberEntity) -> Result<String, ErrorCode> {
        let claims = map_to_refresh_token_claims(self.config.get(), member);
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let secret = &self.config.get().jwt_refresh_token_secret;
        encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|_| ErrorCode::JWT_BUILD_CLAIMS_EXCEPTION)
    }

    fn decode_access_token(&self, token: &str) -> Result<AccessTokenClaims, ErrorCode> {
        let secret = &self.config.get().jwt_access_token_secret;
        tracing::info! {"token:{}", token}
        let mut valid_options = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        valid_options.set_issuer(&[&self.config.get().jwt_issuer]);
        valid_options.set_audience(&[&self.config.get().jwt_audience]);
        let token_data = jsonwebtoken::decode::<AccessTokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &valid_options,
        )
        .map_err(|err| {
            error!("Invalid JWT token: {}, error : {}", token, err.to_string());
            ErrorCode::INVALID_JWT_TOKEN
        });

        match token_data {
            Ok(data) => {
                tracing::info!("token_data: {:?}", data);
                Ok(data.claims)
            }
            Err(err) => {
                error!("Failed to decode access token");
                Err(ErrorCode::INVALID_JWT_TOKEN)
            }
        }
    }

    fn decode_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims, ErrorCode> {
        let secret = &self.config.get().jwt_refresh_token_secret;
        let mut valid_options = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        valid_options.set_issuer(&[&self.config.get().jwt_issuer]);
        valid_options.set_audience(&[&self.config.get().jwt_audience]);

        let token_data = jsonwebtoken::decode::<RefreshTokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &valid_options,
        )
        .map_err(|_| {
            error!("Invalid JWT token: {}", token);
            ErrorCode::INVALID_JWT_TOKEN
        })?;
        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod jwt_tests {

    use super::*;
    use crate::config::*;
    use clap::Parser;
    use dotenvy::dotenv;
    use std::{env, sync::Arc};

    #[test]
    fn create_access_token_test() {
        let cfg = AppConfig::try_parse().unwrap_or_else(|_| AppConfig::parse_from(env::args()));
        let svc = JwtServiceImpl {
            config: Arc::new(ConfigProviderImpl(cfg.clone())),
        };
    }
}
