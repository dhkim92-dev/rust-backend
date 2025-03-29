use axum::{
    routing::post,
    Router,
    Json,
    extract::State
};
use tracing::info;
use crate::application::usecases::auth::{AuthService,AuthUsecase};
use std::sync::{Arc};

use crate::interfaces::auth::dto::{
    LoginRequest,
    LoginResponse
};

async fn login(
    State(auth_service): State<AuthService>,
    Json(req): Json<LoginRequest>
) -> Json<LoginResponse> {
    info!("Login request: {:?}", req);

    let member = auth_service.login_with_email_password(
        crate::application::dto::auth::LoginCommand {
            principal: req.email.address,
            credential: req.password
        }
    ).await.expect("Login failed");

    Json(LoginResponse {
        typ: "Bearer".to_string(),
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
    })
}

pub fn router() -> Router {
    info!("Creating auth router");

    let auth_service = Arc::new(AuthService::new())

    Router::new()
        .route("/api/v1/auth", post(login))
        .layer(axum::Extension())
}
