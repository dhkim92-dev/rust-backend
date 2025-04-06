use crate::{
    common::{
        error::error_code::ErrorCode,
        jwt::{AccessTokenClaims, JwtService},
    },
    di::AppContext,
};
use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use shaku::HasComponent;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct LoginMember {
    pub id: uuid::Uuid,
    pub nickname: String,
    pub email: String,
    pub role: String,
    pub is_activated: bool,
}

impl LoginMember {

    pub fn is_admin(&self) -> bool {
        self.role == "ROLE_ADMIN"
    }

    pub fn is_member(&self) -> bool {
        self.role == "ROLE_MEMBER"
    }

    pub fn is_anonymous(&self) -> bool {
        self.role == "ROLE_ANONYMOUS"
    }

    pub fn from_claims(claims: AccessTokenClaims) -> Self {
        Self {
            id: uuid::Uuid::parse_str(&claims.sub).unwrap(),
            nickname: claims.nickname,
            email: claims.email,
            role: claims.roles[0].clone(),
            is_activated: claims.is_activated,
        }
    }
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord, Clone)]
pub enum SecurityRole {
    Anonymouse,
    Member,
    Admin,
}

#[derive(Debug, Clone)]
pub struct SecurityContext {
    roles: Vec<SecurityRole>,
    member: Option<LoginMember>,
}

pub async fn jwt_authentication_filter(
    State(ctx): State<Arc<AppContext>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ErrorCode> {
    let jwt_service: &dyn JwtService = ctx.resolve_ref();

    // 토큰을 우선 추출해야한다. 토큰이 없다면 인증 절차를 거치지 않는다.
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(String::from);

    if token.is_none() {
        // 토큰이 없다면 로그인하지 않은 상태로 간주한다.
        req.extensions_mut()
            .insert::<SecurityContext>(SecurityContext {
                roles: vec![SecurityRole::Anonymouse],
                member: None,
            });
        return Ok(next.run(req).await);
    }

    // 토큰이 있다면, 토큰을 검증한다.
    let claims = jwt_service.decode_access_token(token.as_ref().unwrap())?;
    // 토큰에서 Claim으로 변환 성공한 경우 인증 정보를 입력한다.
    let login_member = LoginMember::from_claims(claims);

    // 로그인한 사용자의 권한을 SecurityContext에 저장한다.
    req.extensions_mut()
        .insert::<SecurityContext>(SecurityContext {
            roles: vec![SecurityRole::Member],
            member: Some(login_member.clone()),
        });

    Ok(next.run(req).await)
}

pub async fn with_role_member(mut req: Request<Body>, next: Next) -> Result<Response, ErrorCode> {
    info!("with_role_member");
    let exts = req.extensions_mut();
    let ctx = exts
        .get::<SecurityContext>()
        .ok_or(ErrorCode::Unauthorized)?;

    let mut satisfied = false;

    for role in &ctx.roles {
        if *role >= SecurityRole::Member {
            satisfied = true;
            break;
        }
    }

    if satisfied {
        exts.insert::<LoginMember>(ctx.member.clone().unwrap());
        return Ok(next.run(req).await);
    } else {
        // 권한이 없는 경우 403 Forbidden 응답을 반환합니다.
        return Err(ErrorCode::Forbidden);
    }
}

pub async fn with_role_admin(mut req: Request<Body>, next: Next) -> Result<Response, ErrorCode> {
    let exts = req.extensions_mut();
    let ctx = exts
        .get::<SecurityContext>()
        .ok_or(ErrorCode::Unauthorized)?;

    let mut satisfied = false;

    for role in &ctx.roles {
        if *role >= SecurityRole::Member {
            satisfied = true;
            break;
        }
    }

    if satisfied {
        exts.insert::<LoginMember>(ctx.member.clone().unwrap());
        return Ok(next.run(req).await);
    } else {
        // 권한이 없는 경우 403 Forbidden 응답을 반환합니다.
        return Err(ErrorCode::Forbidden);
    }
}
