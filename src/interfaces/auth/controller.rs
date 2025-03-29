use axum::{
    routing::post, Extension, Json, Router,
    response::IntoResponse,
    response::Response,
};
use tracing::info;
use std::sync::{Arc};

use crate::config::AppContext;
use crate::domain::member::repository::MemberQueryRepository;
use crate::{
    interfaces::auth::dto::{LoginRequest, LoginResponse},
    application::dto::auth::{LoginCommand, LoginCommandResponse},
    application::usecases::auth_usecase::{AuthService, AuthUsecase},
};
use crate::common::error::auth_error::AuthError;
use crate::common::error::error_code::ErrorCode;


async fn login(
    Extension(auth_usecase): Extension<Arc<dyn AuthUsecase>>,
    Json(req): Json<LoginRequest>
) -> Result<Json<LoginResponse>, Response> {
    info!("Login request: {:?}", req);
    let command = LoginCommand {
        principal: req.email,
        credential: req.password,
    };

    let result = auth_usecase.login_with_email_password(command)
        .await
        .map_err(|err| {
            err.as_ref().into_response()
        })?;

    let response = LoginResponse {
        typ: result.typ,
        access_token: result.access_token,
        refresh_token: result.refresh_token,
    };
    info!("Login response: {:?}", response);
    Ok(Json(response))
    
}

pub fn router(ctx: Arc<AppContext>) -> Router {
    info!("Creating auth router");

    let member_repository = Arc::new(MemberQueryRepository::new(ctx.clone()));
    let auth_usecase: Arc<dyn AuthUsecase> = Arc::new( AuthService::new(ctx.clone(), member_repository) );

    info!("Auth Port dependency injected");

    Router::new()
        .route("/api/v1/authentication", post(login))
        .layer(Extension(auth_usecase))
} 
