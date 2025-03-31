use std::sync::Arc;

use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenClaims {
    sub: String,
    exp: usize,
    iat: usize,
    iss: String,
    aud: String,
    email: String,
    nickname: String,
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
/* 
pub fn create_access_token(context: Arc<AppContext>, member: &MemberEntity) -> String {

    let claims = AccessTokenClaims {
        sub: member.id.unwrap().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::seconds(context.config.jwt_access_token_expire as i64)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
        iss: context.config.jwt_issuer.clone(),
        aud: context.config.jwt_audience.clone(),
        email: member.email.clone(),
        nickname: member.nickname.clone(),
        roles: vec!(format!("{}_{}", "ROLE", member.role.to_string())),
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    let secret = context.config.jwt_access_token_secret.clone();
    let token = encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| "Failed to create access token".to_string())
        .unwrap_or_else(|_| "Failed to create access token".to_string());
    token
}

pub fn create_refresh_token(context: Arc<AppContext>, member: &MemberEntity) -> String {
    let claims = RefreshTokenClaims {
        sub: member.id.map(|id| id.to_string()).unwrap_or_default(),
        exp: (chrono::Utc::now() + chrono::Duration::seconds(context.config.jwt_refresh_token_expire as i64)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
        iss: context.config.jwt_issuer.clone(),
        aud: context.config.jwt_audience.clone()
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    let secret = context.config.jwt_refresh_token_secret.clone();
    let token = encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| "Failed to create refresh token".to_string())
        .unwrap_or_else(|_| "Failed to create refresh token".to_string());
    token
}

pub fn decode_access_token(context: Arc<AppContext>, token: &str) -> Result<AccessTokenClaims, String> {
    let secret = context.config.jwt_access_token_secret.clone();
    let token_data = jsonwebtoken::decode::<AccessTokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| "Failed to decode access token".to_string())?;
    Ok(token_data.claims)
}

pub fn decode_refresh_token(context: Arc<AppContext>, token: &str) -> Result<RefreshTokenClaims, String> {
    let secret = context.config.jwt_refresh_token_secret.clone();
    let token_data = jsonwebtoken::decode::<RefreshTokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| "Failed to decode refresh token".to_string())?;
    Ok(token_data.claims)
}
 */
